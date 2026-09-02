import { BytesEncoding, ConstantValueNode, isNode } from '@codama/nodes';
import { getBase16Encoder, getBase58Encoder, getBase64Encoder, getUtf8Encoder } from '@solana/codecs-strings';

export function getDiscriminatorBytes(constant: ConstantValueNode): number[] {
    if (isNode(constant.value, 'bytesValueNode')) {
        return encodedStringToBytes(constant.value.data, constant.value.encoding);
    } else if (isNode(constant.value, 'numberValueNode')) {
        const numberType = constant.type;
        if (isNode(numberType, 'numberTypeNode')) {
            return numberToBytes(constant.value.number, numberType.format, numberType.endian);
        }
    } else if (isNode(constant.value, 'stringValueNode') && isNode(constant.type, 'stringTypeNode')) {
        return encodedStringToBytes(constant.value.string, constant.type.encoding);
    }

    throw new Error(`Unsupported discriminator type: ${constant.value.kind}`);
}

function encodedStringToBytes(value: string, encoding: BytesEncoding): number[] {
    switch (encoding) {
        case 'base16':
            return Array.from(getBase16Encoder().encode(value.replace(/^0x/, '')));
        case 'base58':
            return Array.from(getBase58Encoder().encode(value));
        case 'base64':
            return Array.from(getBase64Encoder().encode(value));
        case 'utf8':
            return Array.from(getUtf8Encoder().encode(value));
    }
}

function numberToBytes(num: number, format: string, endian: 'be' | 'le'): number[] {
    let bytes: number[];

    switch (format) {
        case 'u8':
            bytes = [num & 0xff];
            break;
        case 'u16':
            bytes = [num & 0xff, (num >> 8) & 0xff];
            break;
        case 'u32':
            bytes = [num & 0xff, (num >> 8) & 0xff, (num >> 16) & 0xff, (num >> 24) & 0xff];
            break;
        case 'u64': {
            bytes = [];
            let n = num;
            for (let i = 0; i < 8; i++) {
                bytes.push(n & 0xff);
                n = Math.floor(n / 256);
            }
            break;
        }
        default:
            throw new Error(`Unsupported number format: ${format}`);
    }

    return endian === 'be' ? bytes.reverse() : bytes;
}
