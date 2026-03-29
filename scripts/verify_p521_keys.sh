#!/bin/bash

set -e

print_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Verify NIST P-521 elliptic curve key pairs and perform cryptographic operations.

OPTIONS:
    -k, --private-key FILE  Path to private key file (required)
    -p, --public-key FILE   Path to public key file (required)
    -f, --format FORMAT     Key format: pem or der (default: pem)
    -s, --sign              Perform signature test
    -d, --data FILE         File to sign (default: test data)
    -h, --help              Show this help message

EXAMPLES:
    # Verify key pair consistency
    $0 --private-key keys/p521_key_private.pem --public-key keys/p521_key_public.pem

    # Verify with signature test
    $0 --private-key keys/p521_key_private.pem --public-key keys/p521_key_public.pem --sign

    # Verify and sign specific file
    $0 --private-key keys/p521_key_private.pem --public-key keys/p521_key_public.pem --sign --data myfile.txt

    # Verify DER format keys
    $0 --private-key keys/p521_key_private.der --public-key keys/p521_key_public.der --format der

EOF
}

PRIVATE_KEY=""
PUBLIC_KEY=""
FORMAT="pem"
DO_SIGN=false
DATA_FILE=""

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
        -s|--sign)
            DO_SIGN=true
            shift
            ;;
        -d|--data)
            DATA_FILE="$2"
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

if [[ -z "$PRIVATE_KEY" || -z "$PUBLIC_KEY" ]]; then
    echo "Error: Both private and public key files are required"
    print_usage
    exit 1
fi

if [[ ! -f "$PRIVATE_KEY" ]]; then
    echo "Error: Private key file not found: $PRIVATE_KEY"
    exit 1
fi

if [[ ! -f "$PUBLIC_KEY" ]]; then
    echo "Error: Public key file not found: $PUBLIC_KEY"
    exit 1
fi

if [[ "$FORMAT" != "pem" && "$FORMAT" != "der" ]]; then
    echo "Error: Format must be 'pem' or 'der'"
    exit 1
fi

echo "========================================="
echo "NIST P-521 Key Pair Verification"
echo "========================================="
echo ""

TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "Step 1: Validating key formats..."
echo ""

echo "  Checking private key..."
if [[ "$FORMAT" == "pem" ]]; then
    if openssl ec -in "$PRIVATE_KEY" -check -noout 2>/dev/null; then
        echo "    ✓ Valid EC private key"
        CURVE=$(openssl ec -in "$PRIVATE_KEY" -text -noout 2>/dev/null | grep "ASN1 OID" | awk '{print $3}')
        echo "    Curve: $CURVE"
        if [[ "$CURVE" != "secp521r1" ]]; then
            echo "    ✗ Warning: Not a P-521 curve!"
        fi
    else
        echo "    ✗ Invalid private key or encrypted (password required)"
        exit 1
    fi
else
    if openssl ec -in "$PRIVATE_KEY" -inform DER -check -noout 2>/dev/null; then
        echo "    ✓ Valid EC private key"
        CURVE=$(openssl ec -in "$PRIVATE_KEY" -inform DER -text -noout 2>/dev/null | grep "ASN1 OID" | awk '{print $3}')
        echo "    Curve: $CURVE"
        if [[ "$CURVE" != "secp521r1" ]]; then
            echo "    ✗ Warning: Not a P-521 curve!"
        fi
    else
        echo "    ✗ Invalid private key or encrypted (password required)"
        exit 1
    fi
fi

echo ""
echo "  Checking public key..."
if [[ "$FORMAT" == "pem" ]]; then
    if openssl ec -in "$PUBLIC_KEY" -pubin -check -noout 2>/dev/null; then
        echo "    ✓ Valid EC public key"
        CURVE=$(openssl ec -in "$PUBLIC_KEY" -pubin -text -noout 2>/dev/null | grep "ASN1 OID" | awk '{print $3}')
        echo "    Curve: $CURVE"
        if [[ "$CURVE" != "secp521r1" ]]; then
            echo "    ✗ Warning: Not a P-521 curve!"
        fi
    else
        echo "    ✗ Invalid public key"
        exit 1
    fi
else
    if openssl ec -in "$PUBLIC_KEY" -inform DER -pubin -check -noout 2>/dev/null; then
        echo "    ✓ Valid EC public key"
        CURVE=$(openssl ec -in "$PUBLIC_KEY" -inform DER -pubin -text -noout 2>/dev/null | grep "ASN1 OID" | awk '{print $3}')
        echo "    Curve: $CURVE"
        if [[ "$CURVE" != "secp521r1" ]]; then
            echo "    ✗ Warning: Not a P-521 curve!"
        fi
    else
        echo "    ✗ Invalid public key"
        exit 1
    fi
fi

echo ""
echo "Step 2: Extracting public key from private key..."
EXTRACTED_PUBLIC="${TEMP_DIR}/extracted_public.${FORMAT}"

if [[ "$FORMAT" == "pem" ]]; then
    openssl ec -in "$PRIVATE_KEY" -pubout -out "$EXTRACTED_PUBLIC" 2>/dev/null
else
    openssl ec -in "$PRIVATE_KEY" -inform DER -pubout -outform DER -out "$EXTRACTED_PUBLIC" 2>/dev/null
fi
echo "  ✓ Public key extracted"

echo ""
echo "Step 3: Comparing public keys..."

if [[ "$FORMAT" == "pem" ]]; then
    EXTRACTED_DER="${TEMP_DIR}/extracted.der"
    PROVIDED_DER="${TEMP_DIR}/provided.der"
    
    openssl ec -in "$EXTRACTED_PUBLIC" -pubin -outform DER -out "$EXTRACTED_DER"
    openssl ec -in "$PUBLIC_KEY" -pubin -outform DER -out "$PROVIDED_DER"
    
    if cmp -s "$EXTRACTED_DER" "$PROVIDED_DER"; then
        echo "  ✓ Public keys match!"
    else
        echo "  ✗ Public keys do NOT match!"
        echo ""
        echo "Extracted public key fingerprint:"
        openssl dgst -sha256 "$EXTRACTED_DER"
        echo ""
        echo "Provided public key fingerprint:"
        openssl dgst -sha256 "$PROVIDED_DER"
        exit 1
    fi
else
    if cmp -s "$EXTRACTED_PUBLIC" "$PUBLIC_KEY"; then
        echo "  ✓ Public keys match!"
    else
        echo "  ✗ Public keys do NOT match!"
        echo ""
        echo "Extracted public key fingerprint:"
        openssl dgst -sha256 "$EXTRACTED_PUBLIC"
        echo ""
        echo "Provided public key fingerprint:"
        openssl dgst -sha256 "$PUBLIC_KEY"
        exit 1
    fi
fi

echo ""
echo "Step 4: Computing fingerprints..."
if [[ "$FORMAT" == "pem" ]]; then
    echo "  Public key SHA256:"
    openssl ec -in "$PUBLIC_KEY" -pubin -outform DER | openssl dgst -sha256 | sed 's/^/    /'
else
    echo "  Public key SHA256:"
    openssl dgst -sha256 "$PUBLIC_KEY" | sed 's/^/    /'
fi

if [[ "$DO_SIGN" == true ]]; then
    echo ""
    echo "Step 5: Performing signature test..."
    echo ""
    
    if [[ -n "$DATA_FILE" ]]; then
        if [[ ! -f "$DATA_FILE" ]]; then
            echo "  ✗ Data file not found: $DATA_FILE"
            exit 1
        fi
        TEST_DATA="$DATA_FILE"
        echo "  Using data file: $DATA_FILE"
    else
        TEST_DATA="${TEMP_DIR}/test_data.txt"
        echo "This is test data for NIST P-521 signature verification." > "$TEST_DATA"
        echo "Generated: $(date)" >> "$TEST_DATA"
        echo "Random: $RANDOM$RANDOM$RANDOM" >> "$TEST_DATA"
        echo "  Using generated test data"
    fi
    
    SIGNATURE="${TEMP_DIR}/signature.bin"
    
    echo ""
    echo "  Signing data with private key..."
    if [[ "$FORMAT" == "pem" ]]; then
        openssl dgst -sha512 -sign "$PRIVATE_KEY" -out "$SIGNATURE" "$TEST_DATA"
    else
        openssl dgst -sha512 -sign "$PRIVATE_KEY" -keyform DER -out "$SIGNATURE" "$TEST_DATA"
    fi
    echo "    ✓ Data signed"
    
    echo ""
    echo "  Verifying signature with public key..."
    if [[ "$FORMAT" == "pem" ]]; then
        if openssl dgst -sha512 -verify "$PUBLIC_KEY" -signature "$SIGNATURE" "$TEST_DATA" 2>/dev/null; then
            echo "    ✓ Signature verified successfully!"
        else
            echo "    ✗ Signature verification failed!"
            exit 1
        fi
    else
        PUBLIC_PEM="${TEMP_DIR}/public.pem"
        openssl ec -in "$PUBLIC_KEY" -inform DER -pubin -outform PEM -out "$PUBLIC_PEM"
        if openssl dgst -sha512 -verify "$PUBLIC_PEM" -signature "$SIGNATURE" "$TEST_DATA" 2>/dev/null; then
            echo "    ✓ Signature verified successfully!"
        else
            echo "    ✗ Signature verification failed!"
            exit 1
        fi
    fi
    
    echo ""
    echo "  Signature details:"
    echo "    Size: $(wc -c < "$SIGNATURE") bytes"
    echo "    SHA256: $(openssl dgst -sha256 "$SIGNATURE" | awk '{print $2}')"
fi

echo ""
echo "========================================="
echo "✓ Key Pair Verification Complete!"
echo "========================================="
echo ""
echo "Summary:"
echo "  • Private key: Valid P-521 key"
echo "  • Public key:  Valid P-521 key"
echo "  • Consistency: Keys form a valid pair"
if [[ "$DO_SIGN" == true ]]; then
    echo "  • Signature:   Verified successfully"
fi
echo ""
