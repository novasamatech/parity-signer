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
    public void blake2s(String data, Promise promise) {
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
    public void randomPhrase(Promise promise) {
        promise.resolve(ethkeyRandomPhrase());
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

    private static native String ethkeyBrainwalletAddress(String seed);
    private static native String ethkeyBrainwalletBIP39Address(String seed);
    private static native String ethkeyBrainwalletSign(String seed, String message);
    private static native String ethkeyRlpItem(String data, int position);
    private static native String ethkeyKeccak(String data);
    private static native String ethkeyBlake(String data);
    private static native String ethkeyEthSign(String data);
    private static native String ethkeyBlockiesIcon(String seed);
    private static native String ethkeyRandomPhrase();
    private static native String ethkeyEncryptData(String data, String password);
    private static native String ethkeyDecryptData(String data, String password);
    private static native String ethkeyQrCode(String data);
    private static native String ethkeyQrCodeHex(String data);
    private static native String substrateBrainwalletAddress(String seed, int prefix);
    private static native String substrateBrainwalletSign(String seed, String message);
}
