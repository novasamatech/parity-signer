package io.parity.signer;

import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Promise;

public class SubstrateSignModule extends ReactContextBaseJavaModule {

    private final ReactApplicationContext reactContext;

    static {
        System.loadLibrary("signer");
    }

    public SubstrateSignModule(ReactApplicationContext reactContext) {
        super(reactContext);
        this.reactContext = reactContext;
    }

    private void rejectWithException(Promise promise, String code, Exception e) {
        String[] sp = e.getMessage().split(": ");
        String s = sp[sp.length - 1].trim().replace("\"", "");
        promise.reject(code, s);
    }

    @Override
    public String getName() {
        return "SubstrateSign";
    }

    @ReactMethod
    public void brainWalletAddress(String seed, Promise promise) {
        promise.resolve(ethkeyBrainwalletAddress(seed));
    }

    @ReactMethod
    public void brainWalletBIP39Address(String seed, Promise promise) {
        try {
            promise.resolve(ethkeyBrainwalletBIP39Address(seed));
        } catch (Exception e) {
            rejectWithException(promise, "brainwallet bip39 address", e);
        }
    }

    @ReactMethod
    public void brainWalletSign(String seed, String message, Promise promise) {
        try {
            promise.resolve(ethkeyBrainwalletSign(seed, message));
        } catch (Exception e) {
            rejectWithException(promise, "brainwallet sign", e);
        }
    }

    @ReactMethod
    public void rlpItem(String rlp, int position, Promise promise) {
        try {
            promise.resolve(ethkeyRlpItem(rlp, position));
        } catch (Exception e) {
            rejectWithException(promise, "rlp item", e);
        }
    }

    @ReactMethod
    public void keccak(String data, Promise promise) {
        try {
            promise.resolve(ethkeyKeccak(data));
        } catch (Exception e) {
            rejectWithException(promise, "keccak", e);
        }
    }

    @ReactMethod
    public void blake2b(String data, Promise promise) {
        try {
            promise.resolve(ethkeyBlake(data));
        } catch (Exception e) {
            rejectWithException(promise, "blake2b", e);
        }
    }

    @ReactMethod
    public void ethSign(String data, Promise promise) {
        promise.resolve(ethkeyEthSign(data));
    }

    @ReactMethod
    public void blockiesIcon(String seed, Promise promise) {
        promise.resolve(ethkeyBlockiesIcon(seed));
    }

    @ReactMethod
    public void randomPhrase(int wordsNumber, Promise promise) {
        promise.resolve(ethkeyRandomPhrase(wordsNumber));
    }

    @ReactMethod
    public void encryptData(String data, String password, Promise promise) {
        promise.resolve(ethkeyEncryptData(data, password));
    }

    @ReactMethod
    public void decryptData(String data, String password, Promise promise) {
        try {
            promise.resolve(ethkeyDecryptData(data, password));
        } catch (Exception e) {
            rejectWithException(promise, "decrypt data", e);
        }
    }

    @ReactMethod
    public void qrCode(String data, Promise promise) {
        try {
            promise.resolve(ethkeyQrCode(data));
        } catch (Exception e) {
            rejectWithException(promise, "qr code", e);
        }
    }

    @ReactMethod
    public void qrCodeHex(String data, Promise promise) {
        try {
            promise.resolve(ethkeyQrCodeHex(data));
        } catch (Exception e) {
            rejectWithException(promise, "qr code hex", e);
        }
    }

    @ReactMethod
    public void substrateAddress(String seed, int prefix, Promise promise) {
        try {
            promise.resolve(substrateBrainwalletAddress(seed, prefix));
        } catch (Exception e) {
            rejectWithException(promise, "substrate address", e);
        }
    }

    @ReactMethod
    public void substrateSign(String seed, String message, Promise promise) {
        try {
            promise.resolve(substrateBrainwalletSign(seed, message));
        } catch (Exception e) {
            rejectWithException(promise, "substrate sign", e);
        }
    }

    @ReactMethod
    public void schnorrkelVerify(String seed, String message, String signature, Promise promise) {
        try {
            promise.resolve(schnorrkelVerify(seed, message, signature));
        } catch (Exception e) {
            rejectWithException(promise, "schnorrkel verify", e);
        }
    }

    @ReactMethod
    public void decryptDataRef(String data, String password, Promise promise) {
        try {
            // `long` is incompatible with the bridge so pass as a double
            double d = Double.longBitsToDouble(ethkeyDecryptDataRef(data, password));
            if (Double.isNaN(d)) {
                promise.reject("reference is nan", "reference is nan");
            } else {
                promise.resolve(d);
            }
        } catch (Exception e) {
            rejectWithException(promise, "decrypt data ref", e);
        }
    }

    @ReactMethod
    public void destroyDataRef(double data_ref, Promise promise) {
        try {
            ethkeyDestroyDataRef(Double.doubleToRawLongBits(data_ref));
            promise.resolve(0);
        } catch (Exception e) {
            rejectWithException(promise, "destroy data ref", e);
        }
    }

    @ReactMethod
    public void brainWalletSignWithRef(double seed_ref, String message, Promise promise) {
        try {
            promise.resolve(ethkeyBrainwalletSignWithRef(Double.doubleToRawLongBits(seed_ref), message));
        } catch (Exception e) {
            rejectWithException(promise, "brainwallet sign with ref", e);
        }
    }

    @ReactMethod
    public void substrateSignWithRef(double seed_ref, String suriSuffix, String message, Promise promise) {
        try {
            String s = ethkeySubstrateBrainwalletSignWithRef(Double.doubleToRawLongBits(seed_ref), suriSuffix, message);
            promise.resolve(s);
        } catch (Exception e) {
            rejectWithException(promise, "substrate sign with ref", e);
        }
    }

    @ReactMethod
    public void brainWalletAddressWithRef(double seedRef, Promise promise) {
        try {
            String s = ethkeyBrainWalletAddressWithRef(Double.doubleToRawLongBits(seedRef));
            promise.resolve(s);
        } catch (Exception e) {
            rejectWithException(promise, "brainwallet address with ref", e);
        }
    }

    @ReactMethod
    public void substrateAddressWithRef(double seedRef, String suriSuffix, int prefix, Promise promise) {
        try {
            String substrateAddress = ethkeySubstrateWalletAddressWithRef(Double.doubleToRawLongBits(seedRef), suriSuffix, prefix);
            promise.resolve(substrateAddress);
        } catch (Exception e) {
            rejectWithException(promise, "substrate address with ref", e);
        }
    }

    @ReactMethod
    public void substrateSecretWithRef(double seedRef, String suriSuffix, Promise promise) {
        try {
            String derivedSubstrateSecret = ethkeySubstrateMiniSecretKeyWithRef(Double.doubleToRawLongBits(seedRef), suriSuffix);
            promise.resolve(derivedSubstrateSecret);
        } catch (Exception e) {
            rejectWithException(promise, "substrate secret", e);
        }
    }

    @ReactMethod
    public void substrateSecret(String suri, Promise promise) {
        try {
            String derivedSubstrateSecret = ethkeySubstrateMiniSecretKey(suri);
            promise.resolve(derivedSubstrateSecret);
        } catch (Exception e) {
            rejectWithException(promise, "substrate secret with ref", e);
        }
    }

    @ReactMethod
    public void tryDecodeQrSequence(int size, int chunkSize, String data, Promise promise) {
        try {
            String decoded = qrparserTryDecodeQrSequence(size, chunkSize, data);
            promise.resolve(decoded);
        } catch (Exception e) {
            rejectWithException(promise, "try to decode qr goblet", e);
        }
    }

    @ReactMethod
    public void generateMetadataHandle(String metadata, Promise promise) {
    	promise.resolve(metadataGenerateMetadataHandle(metadata));
    }

    private static native String ethkeyBrainwalletAddress(String seed);
    private static native String ethkeyBrainwalletBIP39Address(String seed);
    private static native String ethkeyBrainwalletSign(String seed, String message);
    private static native String ethkeyRlpItem(String data, int position);
    private static native String ethkeyKeccak(String data);
    private static native String ethkeyBlake(String data);
    private static native String ethkeyEthSign(String data);
    private static native String ethkeyBlockiesIcon(String seed);
    private static native String ethkeyRandomPhrase(int wordsNumber);
    private static native String ethkeyEncryptData(String data, String password);
    private static native String ethkeyDecryptData(String data, String password);
    private static native String ethkeyQrCode(String data);
    private static native String ethkeyQrCodeHex(String data);
    private static native String substrateBrainwalletAddress(String seed, int prefix);
    private static native String substrateBrainwalletSign(String seed, String message);
    private static native boolean schnorrkelVerify(String seed, String message, String signature);
    private static native long ethkeyDecryptDataRef(String data, String password);
    private static native void ethkeyDestroyDataRef(long data_ref);
    private static native String ethkeyBrainwalletSignWithRef(long seed_ref, String message);
    private static native String ethkeySubstrateBrainwalletSignWithRef(long seed_ref, String suriSuffix, String message);
    private static native String ethkeySubstrateWalletAddressWithRef(long seedRef, String suriSuffix, int prefix);
    private static native String ethkeyBrainWalletAddressWithRef(long seedRef);
    private static native String ethkeySubstrateMiniSecretKey(String suri);
    private static native String ethkeySubstrateMiniSecretKeyWithRef(long seedRef, String suriSuffix);
    private static native String qrparserTryDecodeQrSequence(int size, int chunkSize, String data);
    private static native String metadataGenerateMetadataHandle(String metadata);
}
