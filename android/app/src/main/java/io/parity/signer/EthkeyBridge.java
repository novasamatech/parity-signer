// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

package io.parity.signer;

import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Promise;

/**
 * Created by marek on 20/02/2017.
 */

public class EthkeyBridge extends ReactContextBaseJavaModule {
    static {
        System.loadLibrary("signer");
    }

    @Override
    public String getName() {
        return "EthkeyBridge";
    }

    public EthkeyBridge(ReactApplicationContext reactContext) {
        super(reactContext);
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
            promise.reject("invalid phrase", "invalid phrase");
        }
    }

    @ReactMethod
    public void brainWalletSign(String seed, String message, Promise promise) {
        try {
            promise.resolve(ethkeyBrainwalletSign(seed, message));
        } catch (Exception e) {
            promise.reject("invalid phrase", "invalid phrase");
        }
    }

    @ReactMethod
    public void rlpItem(String rlp, int position, Promise promise) {
        try {
            promise.resolve(ethkeyRlpItem(rlp, position));
        } catch (Exception e) {
            promise.reject("invalid rlp", "invalid rlp");

        }
    }

    @ReactMethod
    public void keccak(String data, Promise promise) {
        try {
            promise.resolve(ethkeyKeccak(data));
        } catch (Exception e) {
            promise.reject("invalid data, expected hex-encoded string", "invalid data, expected hex-encoded string");
        }
    }

    @ReactMethod
    public void blake2b(String data, Promise promise) {
        try {
            promise.resolve(ethkeyBlake(data));
        } catch (Exception e) {
            promise.reject("invalid data, expected hex-encoded string", "invalid data, expected hex-encoded string");
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
            promise.reject("invalid password", "invalid password");
        }
    }

    @ReactMethod
    public void qrCode(String data, Promise promise) {
        try {
            promise.resolve(ethkeyQrCode(data));
        } catch (Exception e) {
            promise.reject("failed to create QR code", "failed to create QR code");
        }
    }

    @ReactMethod
    public void qrCodeHex(String data, Promise promise) {
        try {
            promise.resolve(ethkeyQrCodeHex(data));
        } catch (Exception e) {
            promise.reject("failed to create QR code", "failed to create QR code");
        }
    }

    @ReactMethod
    public void substrateAddress(String seed, int prefix, Promise promise) {
        try {
            promise.resolve(substrateBrainwalletAddress(seed, prefix));
        } catch (Exception e) {
            promise.reject("invalid phrase", "invalid phrase");
        }
    }

    @ReactMethod
    public void substrateSign(String seed, String message, Promise promise) {
        try {
            promise.resolve(substrateBrainwalletSign(seed, message));
        } catch (Exception e) {
            promise.reject("invalid phrase", "invalid phrase");
        }
    }

    @ReactMethod
    public void schnorrkelVerify(String seed, String message, String signature, Promise promise) {
        try {
            promise.resolve(schnorrkelVerify(seed, message, signature));
        } catch (Exception e) {
            promise.reject("invalid signature", "invalid signature");
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
			promise.reject("decrypted ref", "decrypted ref");
		}
	}

	@ReactMethod
	public void destroyDataRef(double data_ref, Promise promise) {
		try {
			ethkeyDestroyDataRef(Double.doubleToRawLongBits(data_ref));
			promise.resolve(0);
		} catch (Exception e) {
			promise.reject("destroy ref", "destroy ref");
		}
	}

	@ReactMethod
	public void brainWalletSignWithRef(double seed_ref, String message, Promise promise) {
		try {
			promise.resolve(ethkeyBrainwalletSignWithRef(Double.doubleToRawLongBits(seed_ref), message));
		} catch (Exception e) {
			promise.reject("invalid brain wallet phrase", "invalid brain wallet phrase");
		}
	}

	@ReactMethod
	public void substrateSignWithRef(double seed_ref, String suriSuffix, String message, Promise promise) {
		try {
			String s = ethkeySubstrateBrainwalletSignWithRef(Double.doubleToRawLongBits(seed_ref), suriSuffix, message);
			promise.resolve(s);
		} catch (Exception e) {
			promise.reject("invalid substrate phrase", "invalid substrate phrase");
		}
	}

    @ReactMethod
    public void brainWalletAddressWithRef(double seedRef, Promise promise) {
        try {
            String s = ethkeyBrainWalletAddressWithRef(Double.doubleToRawLongBits(seedRef));
            promise.resolve(s);
        } catch (Exception e) {
            promise.reject("invalid substrate phrase", "invalid substrate phrase");
        }
    }

	@ReactMethod
    public void substrateAddressWithRef(double seedRef, String suriSuffix, int prefix, Promise promise) {
        try {
            String substrateAddress = ethkeySubstrateWalletAddressWithRef(Double.doubleToRawLongBits(seedRef), suriSuffix, prefix);
            promise.resolve(substrateAddress);
        } catch (Exception e) {
            promise.reject("invalid suri suffix or prefix", "invalid suri suffix or prefix");
        }
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
}
