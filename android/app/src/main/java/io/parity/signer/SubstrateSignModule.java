package io.parity.signer;

//import java.security.KeyPairGenerator;
import java.security.KeyStore;
//import java.security.KeyPair;

import java.security.NoSuchAlgorithmException;
import java.security.NoSuchProviderException;
import java.security.InvalidAlgorithmParameterException;
import java.security.InvalidKeyException;
import java.security.KeyStoreException;
import java.security.UnrecoverableKeyException;
import java.security.cert.CertificateException;
import java.io.IOException;
import java.io.File;
import java.io.InputStream;
import java.io.OutputStream;
import java.io.FileOutputStream;
import java.util.concurrent.Executor;
import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import javax.crypto.NoSuchPaddingException;
import javax.crypto.BadPaddingException;
import javax.crypto.IllegalBlockSizeException;
import javax.crypto.spec.IvParameterSpec;

import android.os.Looper;
import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.content.res.AssetManager;
import android.provider.Settings;
import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyProperties;
import android.util.Base64;
//import androidx.security.crypto.MasterKey;
import androidx.security.crypto.EncryptedSharedPreferences;
import androidx.fragment.app.FragmentActivity;

import androidx.biometric.BiometricManager;
import androidx.biometric.BiometricPrompt;
import androidx.biometric.BiometricPrompt.PromptInfo;

import com.facebook.react.bridge.AssertionException;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Promise;

public class SubstrateSignModule extends ReactContextBaseJavaModule {

	private final ReactApplicationContext reactContext;
	private final SharedPreferences sharedPreferences;
	private final BiometricPrompt.PromptInfo promptInfo;
	private final String dbname;
	private final BiometricManager biometricManager;
	private Executor executor;
	private BiometricPrompt biometricPrompt;
	private final String KEY_NAME = "SubstrateSignerMasterKey";
	private Object AuthLockAbomination = new Object();
	private final String separator = "-";

    static {
        System.loadLibrary("signer");
    }

	public SubstrateSignModule(ReactApplicationContext reactContext) {
		super(reactContext);
		this.reactContext = reactContext;
		sharedPreferences = reactContext.getSharedPreferences("SubstrateSignKeychain", Context.MODE_PRIVATE);
		dbname = reactContext.getFilesDir().toString();

		promptInfo = new BiometricPrompt.PromptInfo.Builder()
        		.setTitle("Secret seed protection")
	        	.setSubtitle("Please log in")
	        	.setAllowedAuthenticators(BiometricManager.Authenticators.DEVICE_CREDENTIAL)
		        .build();
		biometricManager = BiometricManager.from(reactContext);
		/*if (biometricManager.canAuthenticate(BiometricManager.Authenticators.DEVICE_CREDENTIAL) == BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED) {
			// Prompts the user to create credentials that your app accepts.
			final Intent enrollIntent = new Intent(Settings.ACTION_BIOMETRIC_ENROLL);
			enrollIntent.putExtra(Settings.EXTRA_BIOMETRIC_AUTHENTICATORS_ALLOWED,
				BiometricManager.Authenticators.DEVICE_CREDENTIAL);
			startActivity(enrollIntent);
		}*/
		
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

	private void generateSecretKey(KeyGenParameterSpec keyGenParameterSpec) throws NoSuchAlgorithmException, NoSuchProviderException, InvalidAlgorithmParameterException {
		KeyGenerator keyGenerator = KeyGenerator.getInstance(
			KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore");
		keyGenerator.init(keyGenParameterSpec);
		keyGenerator.generateKey();
	}

	private SecretKey getSecretKey() throws KeyStoreException, NoSuchAlgorithmException, CertificateException, IOException, UnrecoverableKeyException {
		KeyStore keyStore = KeyStore.getInstance("AndroidKeyStore");

		// Before the keystore can be accessed, it must be loaded.
		keyStore.load(null);
		return ((SecretKey)keyStore.getKey(KEY_NAME, null));
	}

	private Cipher getCipher() throws NoSuchAlgorithmException, NoSuchPaddingException {
		return Cipher.getInstance(KeyProperties.KEY_ALGORITHM_AES + "/"
			+ KeyProperties.BLOCK_MODE_CBC + "/"
			+ KeyProperties.ENCRYPTION_PADDING_PKCS7);
	}

	private String decryptSeed(String encryptedSeedRecord) throws NoSuchAlgorithmException, NoSuchPaddingException, KeyStoreException, CertificateException, IOException, UnrecoverableKeyException, InvalidAlgorithmParameterException, BadPaddingException, InvalidKeyException, IllegalBlockSizeException {
		String[] encryptedParts = encryptedSeedRecord.split(separator);
		try {
			Cipher cipher = getCipher();
			SecretKey secretKey = getSecretKey();
			cipher.init(Cipher.DECRYPT_MODE, secretKey, new IvParameterSpec(Base64.decode(encryptedParts[1], Base64.DEFAULT)));		
			String seedPhrase = new String(cipher.doFinal(Base64.decode(encryptedParts[0], Base64.DEFAULT)));
			return seedPhrase;
		} catch (Exception ignored) {
			startAuthentication();
			Cipher cipher = getCipher();
			SecretKey secretKey = getSecretKey();
			cipher.init(Cipher.DECRYPT_MODE, secretKey, new IvParameterSpec(Base64.decode(encryptedParts[1], Base64.DEFAULT)));		
			String seedPhrase = new String(cipher.doFinal(Base64.decode(encryptedParts[0], Base64.DEFAULT)));
			return seedPhrase;
		}
	}

/*	protected BiometricPrompt authenticateWithPrompt(@NonNull final FragmentActivity activity) {
		final BiometricPrompt prompt = new BiometricPrompt(activity, executor, this);
		prompt.authenticate(this.promptInfo);

		return prompt;
	}
*/
	/** Block current NON-main thread and wait for user authentication results. */
	public void waitResult() {
		if (Thread.currentThread() == Looper.getMainLooper().getThread())
			throw new AssertionException("method should not be executed from MAIN thread");

		try {
			synchronized (AuthLockAbomination) {
				AuthLockAbomination.wait();
			}
		} catch (InterruptedException ignored) {
			/* shutdown sequence */
		}
	}

	/** trigger interactive authentication. */
	public void startAuthentication() {
		FragmentActivity activity = (FragmentActivity) getCurrentActivity();
		
		// code can be executed only from MAIN thread
		if (Thread.currentThread() != Looper.getMainLooper().getThread()) {
			activity.runOnUiThread(this::startAuthentication);
			waitResult();
			return;
		}

		executor = reactContext.getMainExecutor();
		biometricPrompt = new BiometricPrompt(
			activity,
			executor, 
			new BiometricPrompt.AuthenticationCallback(){
				@Override
				public void onAuthenticationError(int errorCode,
					CharSequence errString) {
					super.onAuthenticationError(errorCode, errString);
					synchronized (AuthLockAbomination) {
						AuthLockAbomination.notify();
					}

				}

				@Override
				public void onAuthenticationSucceeded(
					BiometricPrompt.AuthenticationResult result) {
					super.onAuthenticationSucceeded(result);
					synchronized (AuthLockAbomination) {
						AuthLockAbomination.notify();
					}
				}

				@Override
				public void onAuthenticationFailed() {
					super.onAuthenticationFailed();
					synchronized (AuthLockAbomination) {
						AuthLockAbomination.notify();
					}
				}
			});
		
		biometricPrompt.authenticate(promptInfo);
	}

	/**
	 * Copy the database asset at the specified path to this app's data directory. If the
	 * asset is a directory, its contents are also copied.
	 *	
	 * @param path
	 * Path to asset, relative to app's assets/database directory.
	 */
	private void copyAsset(String path) throws IOException {
		AssetManager manager = reactContext.getAssets();

		// If we have a directory, we make it and recurse. If a file, we copy its
		// contents.
		try {
			String[] contents = manager.list("database" + path);

			// The documentation suggests that list throws an IOException, but doesn't
			// say under what conditions. It'd be nice if it did so when the path was
			// to a file. That doesn't appear to be the case. If the returned array is
			// null or has 0 length, we assume the path is to a file. This means empty
			// directories will get turned into files.
			if (contents == null || contents.length == 0)
				throw new IOException();

			// Make the directory.
			File dir = new File(dbname, path);
			dir.mkdirs();

			// Recurse on the contents.
			for (String entry : contents) {
				copyAsset(path + "/" + entry);
			}
		} catch (IOException e) {
			copyFileAsset(path);
		}
	}

	/**
	 * Copy the database asset file specified by path to app's data directory. Assumes
	 * parent directories have already been created.
	 *
	 * @param path
	 * Path to asset, relative to app's assets/database directory.
	 */
	private void copyFileAsset(String path) throws IOException {
		AssetManager manager = reactContext.getAssets();
		File file = new File(dbname, path);
		InputStream in = manager.open("database" + path);
		OutputStream out = new FileOutputStream(file);
		byte[] buffer = new byte[1024];
		int read = in.read(buffer);
		while (read != -1) {
			out.write(buffer, 0, read);
			read = in.read(buffer);
		}
		out.close();
		in.close();
	}


	//react native section begin

	@ReactMethod
	public void qrCode(String data, Promise promise) {
		try {
			promise.resolve(ethkeyQrCode(data));
		} catch (Exception e) {
			rejectWithException(promise, "qr code", e);
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

	@ReactMethod
	public void parseTransaction(String transaction, Promise promise) {
		try {
			String decoded = substrateParseTransaction(transaction, dbname);
			promise.resolve(decoded);
		} catch (Exception e) {
			rejectWithException(promise, "transaction parsing", e);
		}
	}

	@ReactMethod
	public void signTransaction(String action, String seedName, String password, Promise promise) {
		try {
			String encryptedSeedRecord = sharedPreferences.getString(seedName, null);
			String seedPhrase = decryptSeed(encryptedSeedRecord);
			String signed = substrateSignTransaction(action, seedPhrase, password, dbname);
			promise.resolve(signed);
		} catch (Exception e) {
			rejectWithException(promise, "transaction signing", e);
		}
	}

	@ReactMethod
	public void handleTransaction(String action, Promise promise) {
		try {
			String signed = substrateSignTransaction(action, "", "", dbname);
			promise.resolve(signed);
		} catch (Exception e) {
			rejectWithException(promise, "transaction handling", e);
		}
	}

	@ReactMethod
	public void developmentTest(String input, Promise promise) {
		try {
			String path = reactContext.getFilesDir().toString();
			String output = substrateDevelopmentTest(path);
			promise.resolve(output);
		} catch (Exception e) {
			rejectWithException(promise, "Rust interface testing error", e);
		}
	}

	//This should be only run once ever!
	//Also this factory resets the Signer upon deletion of local data
	//Should be cryptographically reliable
	//However this was not audited yet
	@ReactMethod
	public void dbInit(Promise promise) {
		try {
			//Move database from assets to internal storage
			//This function is not usable for anything else
			copyAsset("");
			generateSecretKey(new KeyGenParameterSpec.Builder(
				KEY_NAME,
				KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT
			).setBlockModes(KeyProperties.BLOCK_MODE_CBC)
				.setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_PKCS7)
				.setUserAuthenticationParameters(1, KeyProperties.AUTH_DEVICE_CREDENTIAL)
				.setUserAuthenticationRequired(true)
				.build());
			promise.resolve(0);
		} catch (Exception e) {
			rejectWithException(promise, "Database initialization error", e);
		}
	}

	@ReactMethod
	public void getAllNetworksForNetworkSelector(Promise promise) {
		try {
			String allNetworks = dbGetAllNetworksForNetworkSelector(dbname);
			promise.resolve(allNetworks);
		} catch (Exception e) {
			rejectWithException(promise, "Database all networks fetch error", e);
		}
	}
	
	@ReactMethod
	public void getNetwork(String genesisHash, Promise promise) {
		try {
			String network = dbGetNetwork(genesisHash, dbname);
			promise.resolve(network);
		} catch (Exception e) {
			rejectWithException(promise, "Database network fetch error", e);
		}
	}

	@ReactMethod
	public void getAllSeedNames(Promise promise) {
		try {
			String seedNameSet = "[\"" + String.join("\", \"", sharedPreferences.getAll().keySet()) + "\"]";
			promise.resolve(seedNameSet);
		} catch (Exception e) {
			rejectWithException(promise, "Database all seed names fetch error", e);
		}
	}

	@ReactMethod
	public void getRelevantIdentities(String seedName, String genesisHash, Promise promise) {
		try {
			String allSeedNames = dbGetRelevantIdentities(seedName, genesisHash, dbname);
			promise.resolve(allSeedNames);
		} catch (Exception e) {
			rejectWithException(promise, "Database fetch relevant identities error", e);
		}
	}

	@ReactMethod
	public void tryCreateSeed(String seedName, String crypto, int seedLength, Promise promise) {
		try {
			if (sharedPreferences.contains(seedName)) throw new AssertionException("Seed with this name already exists");

			startAuthentication();
			
			Cipher cipher = getCipher();
			SecretKey secretKey = getSecretKey();
			
			String seedPhrase = substrateTryCreateSeed(seedName, crypto, "", seedLength, dbname);
			
			cipher.init(Cipher.ENCRYPT_MODE, secretKey);
			String iv = Base64.encodeToString(cipher.getIV(), Base64.DEFAULT);
			byte[] encryptedSeedBytes = cipher.doFinal(seedPhrase.getBytes());
			String encryptedSeedRecord = Base64.encodeToString(encryptedSeedBytes, Base64.DEFAULT) + separator + iv;

			sharedPreferences.edit().putString(seedName, encryptedSeedRecord).apply();
			
			promise.resolve(seedPhrase + " =|= " + encryptedSeedRecord);
		} catch (Exception e) {
			rejectWithException(promise, "New seed creation failed", e);
		}
	}

	@ReactMethod
	public void tryRecoverSeed(String seedName, String crypto, String seedPhrase, Promise promise) {
		try {
			if (sharedPreferences.contains(seedName)) throw new AssertionException("Seed with this name already exists");

			startAuthentication();
			
			Cipher cipher = getCipher();
			SecretKey secretKey = getSecretKey();
			
			String seedPhraseCheck = substrateTryCreateSeed(seedName, crypto, seedPhrase, 0, dbname);
			
			cipher.init(Cipher.ENCRYPT_MODE, secretKey);
			String iv = Base64.encodeToString(cipher.getIV(), Base64.DEFAULT);
			byte[] encryptedSeedBytes = cipher.doFinal(seedPhraseCheck.getBytes());
			String encryptedSeedRecord = Base64.encodeToString(encryptedSeedBytes, Base64.DEFAULT) + separator + iv;

			sharedPreferences.edit().putString(seedName, encryptedSeedRecord).apply();
			
			promise.resolve(seedPhraseCheck + " =|= " + encryptedSeedRecord);
		} catch (Exception e) {
			rejectWithException(promise, "Seed recovery failed", e);
		}
	}

	@ReactMethod
	public void fetchSeed(String seedName, String pin, Promise promise) {
		try {
			String encryptedSeedRecord = sharedPreferences.getString(seedName, null);
			String seedPhrase = decryptSeed(encryptedSeedRecord);
			promise.resolve(seedPhrase);
		} catch (Exception e) {
			rejectWithException(promise, "Seed fetch failed", e);
		}
	}

	@ReactMethod
	public void tryCreateIdentity(String idName, String seedName, String crypto, String path, String network, Promise promise) {
		try {
			boolean hasPassword = substrateCheckPath(path);
			String encryptedSeedRecord = sharedPreferences.getString(seedName, null);
			String seedPhrase = decryptSeed(encryptedSeedRecord);
			substrateTryCreateIdentity(idName, seedName, seedPhrase, crypto, path, network, hasPassword, dbname);	
			promise.resolve(0);
		} catch (Exception e) {
			rejectWithException(promise, "Identity creation failed", e);
		}
	}

	@ReactMethod
	public void suggestNPlusOne(String path, String seedName, String networkId, Promise promise) {
		try {
			String suggestion = substrateSuggestNPlusOne(path, seedName, networkId, dbname);
			promise.resolve(suggestion);
		} catch (Exception e) {
			rejectWithException(promise, "Can not suggest a path", e);
		}
	}

	@ReactMethod
	public void suggestPathName(String path, Promise promise) {
		String suggestion = substrateSuggestName(path);
		promise.resolve(suggestion);
	}

	@ReactMethod
	public void deleteIdentity(String pubKey, String networkId, Promise promise) {
		try {
			substrateDeleteIdentity(pubKey, networkId, dbname);
			promise.resolve(0);
		} catch (Exception e) {
			rejectWithException(promise, "Can not delete identity", e);
		}
	}

	@ReactMethod
	public void getNetworkSpecs(String networkId, Promise promise) {
		try {
			String networkSpecs = substrateGetNetworkSpecs(networkId, dbname);
			promise.resolve(networkSpecs);
		} catch (Exception e) {
			rejectWithException(promise, "Network settings fetch failure", e);
		}
	}

	//react native section end

	//rust native section begin

	private static native String ethkeyQrCode(String data);
	private static native String qrparserTryDecodeQrSequence(int size, int chunkSize, String data);
	private static native String metadataGenerateMetadataHandle(String metadata);
	private static native String substrateParseTransaction(String transaction, String dbname);
	private static native String substrateSignTransaction(String action, String seedPhrase, String password, String dbname);
	private static native String substrateDevelopmentTest(String input);
	private static native String dbGetAllNetworksForNetworkSelector(String dbname);
	private static native String dbGetNetwork(String genesisHash, String dbname);
	private static native String dbGetRelevantIdentities(String seedName, String genesisHash, String dbname);
	private static native String substrateTryCreateSeed(String seedName, String crypto, String seedPhrase, int seedLength, String dbname);
	private static native String substrateSuggestNPlusOne(String path, String seedName, String networkIdString, String dbname);
	private static native boolean substrateCheckPath(String path);
	private static native void substrateTryCreateIdentity(String idName, String seedName, String seedPhrase, String crypto, String path, String network, boolean hasPassword, String dbname);
	private static native String substrateSuggestName(String path);
	private static native void substrateDeleteIdentity(String pubKey, String network, String dbname);
	private static native String substrateGetNetworkSpecs(String network, String dbname);

	//rust native section end
}
