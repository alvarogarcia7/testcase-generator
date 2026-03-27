#!/bin/bash

set -e

print_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Generate NIST P-521 elliptic curve keys using OpenSSL.

OPTIONS:
    -o, --output-dir DIR    Output directory for keys (default: ./keys)
    -n, --name NAME         Key name prefix (default: p521_key)
    -f, --format FORMAT     Output format: pem or der (default: pem)
    -p, --password          Encrypt private key with password
    -h, --help              Show this help message

EXAMPLES:
    # Generate keys with default settings
    $0

    # Generate keys in specific directory
    $0 --output-dir /path/to/keys

    # Generate keys with custom name
    $0 --name my_server_key

    # Generate password-protected keys
    $0 --password

    # Generate keys in DER format
    $0 --format der

OUTPUT FILES:
    <name>_private.pem    Private key in PEM format
    <name>_public.pem     Public key in PEM format
    <name>_params.pem     EC parameters file
    <name>.info           Key information and fingerprint

EOF
}

OUTPUT_DIR="./keys"
KEY_NAME="p521_key"
FORMAT="pem"
USE_PASSWORD=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -n|--name)
            KEY_NAME="$2"
            shift 2
            ;;
        -f|--format)
            FORMAT="$2"
            shift 2
            ;;
        -p|--password)
            USE_PASSWORD=true
            shift
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

if [[ "$FORMAT" != "pem" && "$FORMAT" != "der" ]]; then
    echo "Error: Format must be 'pem' or 'der'"
    exit 1
fi

mkdir -p "$OUTPUT_DIR"

PRIVATE_KEY="${OUTPUT_DIR}/${KEY_NAME}_private.${FORMAT}"
PUBLIC_KEY="${OUTPUT_DIR}/${KEY_NAME}_public.${FORMAT}"
PARAMS_FILE="${OUTPUT_DIR}/${KEY_NAME}_params.${FORMAT}"
INFO_FILE="${OUTPUT_DIR}/${KEY_NAME}.info"

echo "Generating NIST P-521 elliptic curve keys..."
echo "Output directory: $OUTPUT_DIR"
echo "Key name: $KEY_NAME"
echo "Format: $FORMAT"
echo ""

echo "Step 1: Generating EC parameters..."
if [[ "$FORMAT" == "pem" ]]; then
    openssl ecparam -name secp521r1 -out "$PARAMS_FILE"
else
    openssl ecparam -name secp521r1 -outform DER -out "$PARAMS_FILE"
fi
echo "  ✓ Parameters saved to: $PARAMS_FILE"

echo ""
echo "Step 2: Generating private key..."
if [[ "$USE_PASSWORD" == true ]]; then
    if [[ "$FORMAT" == "pem" ]]; then
        openssl ecparam -name secp521r1 -genkey -out "$PRIVATE_KEY" -param_enc named_curve
        openssl ec -in "$PRIVATE_KEY" -out "${PRIVATE_KEY}.encrypted" -aes256
        mv "${PRIVATE_KEY}.encrypted" "$PRIVATE_KEY"
    else
        TEMP_PEM="${OUTPUT_DIR}/${KEY_NAME}_private_temp.pem"
        openssl ecparam -name secp521r1 -genkey -out "$TEMP_PEM" -param_enc named_curve
        openssl ec -in "$TEMP_PEM" -aes256 -outform DER -out "$PRIVATE_KEY"
        rm "$TEMP_PEM"
    fi
    echo "  ✓ Encrypted private key saved to: $PRIVATE_KEY"
else
    if [[ "$FORMAT" == "pem" ]]; then
        openssl ecparam -name secp521r1 -genkey -out "$PRIVATE_KEY" -param_enc named_curve
    else
        openssl ecparam -name secp521r1 -genkey -outform DER -out "$PRIVATE_KEY" -param_enc named_curve
    fi
    echo "  ✓ Private key saved to: $PRIVATE_KEY"
fi

echo ""
echo "Step 3: Extracting public key..."
if [[ "$FORMAT" == "pem" ]]; then
    openssl ec -in "$PRIVATE_KEY" -pubout -out "$PUBLIC_KEY"
else
    if [[ "$USE_PASSWORD" == true ]]; then
        openssl ec -in "$PRIVATE_KEY" -inform DER -pubout -outform DER -out "$PUBLIC_KEY"
    else
        openssl ec -in "$PRIVATE_KEY" -inform DER -pubout -outform DER -out "$PUBLIC_KEY"
    fi
fi
echo "  ✓ Public key saved to: $PUBLIC_KEY"

echo ""
echo "Step 4: Generating key information..."
{
    echo "NIST P-521 Key Information"
    echo "=========================="
    echo "Generated: $(date)"
    echo "Key Name: $KEY_NAME"
    echo "Format: $FORMAT"
    echo "Encrypted: $USE_PASSWORD"
    echo ""
    echo "Files:"
    echo "  Private Key: $PRIVATE_KEY"
    echo "  Public Key: $PUBLIC_KEY"
    echo "  Parameters: $PARAMS_FILE"
    echo ""
    echo "Private Key Details:"
    echo "--------------------"
    if [[ "$FORMAT" == "pem" ]]; then
        openssl ec -in "$PRIVATE_KEY" -text -noout 2>/dev/null || echo "  (Encrypted - requires password to view)"
    else
        openssl ec -in "$PRIVATE_KEY" -inform DER -text -noout 2>/dev/null || echo "  (Encrypted - requires password to view)"
    fi
    echo ""
    echo "Public Key Details:"
    echo "-------------------"
    if [[ "$FORMAT" == "pem" ]]; then
        openssl ec -in "$PUBLIC_KEY" -pubin -text -noout
    else
        openssl ec -in "$PUBLIC_KEY" -inform DER -pubin -text -noout
    fi
    echo ""
    echo "Public Key Fingerprint (SHA256):"
    echo "---------------------------------"
    if [[ "$FORMAT" == "pem" ]]; then
        openssl ec -in "$PUBLIC_KEY" -pubin -outform DER | openssl dgst -sha256
    else
        openssl dgst -sha256 "$PUBLIC_KEY"
    fi
} > "$INFO_FILE"
echo "  ✓ Key information saved to: $INFO_FILE"

echo ""
echo "========================================="
echo "✓ Key generation complete!"
echo "========================================="
echo ""
echo "Generated files:"
echo "  • Private key: $PRIVATE_KEY"
echo "  • Public key:  $PUBLIC_KEY"
echo "  • Parameters:  $PARAMS_FILE"
echo "  • Info:        $INFO_FILE"
echo ""

if [[ "$USE_PASSWORD" == false ]]; then
    echo "⚠️  WARNING: Private key is NOT encrypted!"
    echo "    Consider using --password flag for production keys."
    echo ""
fi

echo "To view key details:"
if [[ "$FORMAT" == "pem" ]]; then
    echo "  openssl ec -in $PRIVATE_KEY -text -noout"
    echo "  openssl ec -in $PUBLIC_KEY -pubin -text -noout"
else
    echo "  openssl ec -in $PRIVATE_KEY -inform DER -text -noout"
    echo "  openssl ec -in $PUBLIC_KEY -inform DER -pubin -text -noout"
fi
echo ""

echo "To verify the key pair:"
echo "  Run: ./scripts/verify_p521_keys.sh --private $PRIVATE_KEY --public $PUBLIC_KEY --format $FORMAT"
echo ""
