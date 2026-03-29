#!/bin/bash

set -e

print_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Load and verify NIST P-521 elliptic curve keys from files.

OPTIONS:
    -k, --private-key FILE  Path to private key file (required)
    -p, --public-key FILE   Path to public key file (optional)
    -f, --format FORMAT     Key format: pem or der (default: pem)
    -v, --verify            Verify key pair consistency
    -i, --info              Display detailed key information
    -e, --extract-public    Extract public key from private key
    -o, --output FILE       Output file for extracted public key
    -h, --help              Show this help message

EXAMPLES:
    # Load and display private key information
    $0 --private-key keys/p521_key_private.pem --info

    # Verify key pair consistency
    $0 --private-key keys/p521_key_private.pem --public-key keys/p521_key_public.pem --verify

    # Extract public key from private key
    $0 --private-key keys/p521_key_private.pem --extract-public --output new_public.pem

    # Load DER format keys
    $0 --private-key keys/p521_key_private.der --format der --info

EOF
}

PRIVATE_KEY=""
PUBLIC_KEY=""
FORMAT="pem"
DO_VERIFY=false
SHOW_INFO=false
EXTRACT_PUBLIC=false
OUTPUT_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -k|--private-key)
            PRIVATE_KEY="$2"
            shift 2
            ;;
        -p|--public-key)
            PUBLIC_KEY="$2"
            shift 2
            ;;
        -f|--format)
            FORMAT="$2"
            shift 2
            ;;
        -v|--verify)
            DO_VERIFY=true
            shift
            ;;
        -i|--info)
            SHOW_INFO=true
            shift
            ;;
        -e|--extract-public)
            EXTRACT_PUBLIC=true
            shift
            ;;
        -o|--output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo "Error: Unknown option $1"
            print_usage
            exit 1
            ;;
    esac
done

if [[ -z "$PRIVATE_KEY" ]]; then
    echo "Error: Private key file is required"
    print_usage
    exit 1
fi

if [[ ! -f "$PRIVATE_KEY" ]]; then
    echo "Error: Private key file not found: $PRIVATE_KEY"
    exit 1
fi

if [[ -n "$PUBLIC_KEY" && ! -f "$PUBLIC_KEY" ]]; then
    echo "Error: Public key file not found: $PUBLIC_KEY"
    exit 1
fi

if [[ "$FORMAT" != "pem" && "$FORMAT" != "der" ]]; then
    echo "Error: Format must be 'pem' or 'der'"
    exit 1
fi

echo "Loading NIST P-521 keys..."
echo ""

verify_key_format() {
    local key_file="$1"
    local key_type="$2"
    local format="$3"
    
    echo "Checking $key_type key format..."
    
    if [[ "$format" == "pem" ]]; then
        if [[ "$key_type" == "private" ]]; then
            if openssl ec -in "$key_file" -check -noout 2>/dev/null; then
                echo "  ✓ Valid P-521 private key"
                return 0
            fi
        else
            if openssl ec -in "$key_file" -pubin -check -noout 2>/dev/null; then
                echo "  ✓ Valid P-521 public key"
                return 0
            fi
        fi
    else
        if [[ "$key_type" == "private" ]]; then
            if openssl ec -in "$key_file" -inform DER -check -noout 2>/dev/null; then
                echo "  ✓ Valid P-521 private key"
                return 0
            fi
        else
            if openssl ec -in "$key_file" -inform DER -pubin -check -noout 2>/dev/null; then
                echo "  ✓ Valid P-521 public key"
                return 0
            fi
        fi
    fi
    
    echo "  ✗ Invalid or encrypted $key_type key"
    return 1
}

verify_key_format "$PRIVATE_KEY" "private" "$FORMAT"
PRIVATE_KEY_VALID=$?

if [[ -n "$PUBLIC_KEY" ]]; then
    verify_key_format "$PUBLIC_KEY" "public" "$FORMAT"
    PUBLIC_KEY_VALID=$?
fi

echo ""

if [[ "$SHOW_INFO" == true ]]; then
    echo "========================================="
    echo "Private Key Information"
    echo "========================================="
    if [[ "$FORMAT" == "pem" ]]; then
        openssl ec -in "$PRIVATE_KEY" -text -noout 2>/dev/null || {
            echo "Cannot display key details (may be encrypted)"
            echo "Private key file: $PRIVATE_KEY"
        }
    else
        openssl ec -in "$PRIVATE_KEY" -inform DER -text -noout 2>/dev/null || {
            echo "Cannot display key details (may be encrypted)"
            echo "Private key file: $PRIVATE_KEY"
        }
    fi
    echo ""
    
    if [[ -n "$PUBLIC_KEY" ]]; then
        echo "========================================="
        echo "Public Key Information"
        echo "========================================="
        if [[ "$FORMAT" == "pem" ]]; then
            openssl ec -in "$PUBLIC_KEY" -pubin -text -noout
        else
            openssl ec -in "$PUBLIC_KEY" -inform DER -pubin -text -noout
        fi
        echo ""
    fi
fi

if [[ "$EXTRACT_PUBLIC" == true ]]; then
    if [[ -z "$OUTPUT_FILE" ]]; then
        OUTPUT_FILE="${PRIVATE_KEY%.${FORMAT}}_extracted_public.${FORMAT}"
    fi
    
    echo "========================================="
    echo "Extracting Public Key"
    echo "========================================="
    
    if [[ "$FORMAT" == "pem" ]]; then
        if openssl ec -in "$PRIVATE_KEY" -pubout -out "$OUTPUT_FILE" 2>/dev/null; then
            echo "  ✓ Public key extracted to: $OUTPUT_FILE"
        else
            echo "  ✗ Failed to extract public key (key may be encrypted)"
            exit 1
        fi
    else
        if openssl ec -in "$PRIVATE_KEY" -inform DER -pubout -outform DER -out "$OUTPUT_FILE" 2>/dev/null; then
            echo "  ✓ Public key extracted to: $OUTPUT_FILE"
        else
            echo "  ✗ Failed to extract public key (key may be encrypted)"
            exit 1
        fi
    fi
    echo ""
fi

if [[ "$DO_VERIFY" == true ]]; then
    if [[ -z "$PUBLIC_KEY" ]]; then
        echo "Error: Public key required for verification"
        echo "Use --extract-public to extract it from the private key first"
        exit 1
    fi
    
    echo "========================================="
    echo "Verifying Key Pair Consistency"
    echo "========================================="
    
    TEMP_DIR=$(mktemp -d)
    trap "rm -rf $TEMP_DIR" EXIT
    
    TEMP_PUBLIC="${TEMP_DIR}/extracted_public.${FORMAT}"
    
    echo "Extracting public key from private key..."
    if [[ "$FORMAT" == "pem" ]]; then
        if ! openssl ec -in "$PRIVATE_KEY" -pubout -out "$TEMP_PUBLIC" 2>/dev/null; then
            echo "  ✗ Failed to extract public key (private key may be encrypted)"
            exit 1
        fi
    else
        if ! openssl ec -in "$PRIVATE_KEY" -inform DER -pubout -outform DER -out "$TEMP_PUBLIC" 2>/dev/null; then
            echo "  ✗ Failed to extract public key (private key may be encrypted)"
            exit 1
        fi
    fi
    
    echo "Comparing public keys..."
    
    if [[ "$FORMAT" == "pem" ]]; then
        EXTRACTED_PUB_DER="${TEMP_DIR}/extracted.der"
        PROVIDED_PUB_DER="${TEMP_DIR}/provided.der"
        
        openssl ec -in "$TEMP_PUBLIC" -pubin -outform DER -out "$EXTRACTED_PUB_DER"
        openssl ec -in "$PUBLIC_KEY" -pubin -outform DER -out "$PROVIDED_PUB_DER"
        
        if cmp -s "$EXTRACTED_PUB_DER" "$PROVIDED_PUB_DER"; then
            echo "  ✓ Key pair is consistent!"
            echo ""
            echo "Public key fingerprint (SHA256):"
            openssl dgst -sha256 "$EXTRACTED_PUB_DER"
        else
            echo "  ✗ Key pair mismatch!"
            echo "  The provided public key does not match the private key."
            exit 1
        fi
    else
        if cmp -s "$TEMP_PUBLIC" "$PUBLIC_KEY"; then
            echo "  ✓ Key pair is consistent!"
            echo ""
            echo "Public key fingerprint (SHA256):"
            openssl dgst -sha256 "$PUBLIC_KEY"
        else
            echo "  ✗ Key pair mismatch!"
            echo "  The provided public key does not match the private key."
            exit 1
        fi
    fi
    echo ""
fi

if [[ "$SHOW_INFO" == false && "$DO_VERIFY" == false && "$EXTRACT_PUBLIC" == false ]]; then
    echo "Keys loaded successfully."
    echo ""
    echo "Use --info to display key details"
    echo "Use --verify to verify key pair consistency"
    echo "Use --extract-public to extract public key from private key"
    echo ""
fi

echo "✓ Done"
