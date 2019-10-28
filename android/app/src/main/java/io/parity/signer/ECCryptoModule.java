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

import android.os.Build;
import android.util.Base64;
import android.util.Log;
import com.facebook.react.bridge.*;
import com.google.crypto.tink.HybridDecrypt;
import com.google.crypto.tink.HybridEncrypt;
import com.google.crypto.tink.KeysetHandle;
import com.google.crypto.tink.hybrid.HybridConfig;
import com.google.crypto.tink.hybrid.HybridDecryptFactory;
import com.google.crypto.tink.hybrid.HybridEncryptFactory;
import com.google.crypto.tink.hybrid.HybridKeyTemplates;
import com.google.crypto.tink.integration.android.AndroidKeysetManager;

import java.io.IOException;
import java.security.GeneralSecurityException;
import java.util.HashMap;
import java.util.Map;

public class ECCryptoModule extends ReactContextBaseJavaModule {
    private static final String KEYSTORE_PROVIDER_NAME = "parity.singer.keystore";
    private static final String SHARED_PREF_FILE_NAME = "parity.signer.shared.pref";
    private static final String ANDORID_KEYSTORE_PREFIX = "android-keystore://";
    private static final Map<Integer, String> sizeToName = new HashMap<Integer, String>();
    private static final Map<Integer, byte[]> sizeToHead = new HashMap<Integer, byte[]>();

    private final boolean isModern = android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.M;
    private KeysetHandle keysetHandle;

    public ECCryptoModule(ReactApplicationContext reactContext) {
        super(reactContext);
        try {
            HybridConfig.init();
        } catch (GeneralSecurityException ex) {
            Log.e("init ECCrypto Module", "ERR", ex);
        }
    }

    private static String getKeyUri(String keyLabel) {
        return ANDORID_KEYSTORE_PREFIX + keyLabel;
    }

    private static String toBase64(byte[] bytes) {
        return Base64.encodeToString(bytes, Base64.NO_WRAP);
    }

    private static byte[] fromBase64(String str) {
        return Base64.decode(str, Base64.NO_WRAP);
    }

    @Override
    public String getName() {
        return "ECCrypto";
    }

    @ReactMethod
    public void encrypt(ReadableMap map, Promise promise) {
        try {
            KeysetHandle keysetHandle = getOrGenerateNewKeysetHandle(map.getString("label"));
            KeysetHandle publicKeysetHandle = keysetHandle.getPublicKeysetHandle();
            String clearTextString = map.getString("data");
            byte[] plainText = clearTextString.getBytes("UTF-8");
            byte[] contextInfo = fromBase64(SHARED_PREF_FILE_NAME);

            HybridEncrypt hybridEncrypt = HybridEncryptFactory.getPrimitive(publicKeysetHandle);
            byte[] cipherText = hybridEncrypt.encrypt(plainText, contextInfo);

            promise.resolve(toBase64(cipherText));
        } catch (Exception e) {
            Log.e("ECCrypto", "encryption error", e);
            promise.reject("ECCrypto error", e);
            return;
        }
    }

    @ReactMethod
    public void decrypt(ReadableMap map, Promise promise) {
        try {
            KeysetHandle keysetHandle = getOrGenerateNewKeysetHandle(map.getString("label"));
            String cipherTextString = map.getString("data");
            byte[] cipherText = fromBase64(cipherTextString);
            byte[] contextInfo = fromBase64(SHARED_PREF_FILE_NAME);

            HybridDecrypt hybridDecrypt = HybridDecryptFactory.getPrimitive(keysetHandle);
            byte[] clearText = hybridDecrypt.decrypt(cipherText, contextInfo);

            promise.resolve(new String(clearText, "UTF-8"));
        } catch (Exception e) {
            Log.e("ECCrypto", "decrypt error", e);
            promise.reject("ECCrypto error", e);
            return;
        }
    }

    private KeysetHandle getOrGenerateNewKeysetHandle(String keyUri) throws IOException, GeneralSecurityException {
        return new AndroidKeysetManager.Builder()
                .withSharedPref(getReactApplicationContext(), KEYSTORE_PROVIDER_NAME, SHARED_PREF_FILE_NAME)
                .withKeyTemplate(HybridKeyTemplates.ECIES_P256_HKDF_HMAC_SHA256_AES128_CTR_HMAC_SHA256)
                .withMasterKeyUri(getKeyUri(keyUri))
                .build()
                .getKeysetHandle();
    }
}
