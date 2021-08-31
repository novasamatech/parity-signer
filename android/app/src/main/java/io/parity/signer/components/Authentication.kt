package io.parity.signer.components

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.provider.Settings
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Log
import android.widget.Toast
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricManager.Authenticators.DEVICE_CREDENTIAL
import androidx.biometric.BiometricPrompt
import androidx.compose.ui.platform.LocalContext

import androidx.core.app.ActivityCompat.startActivityForResult
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.SecretKey


class Authentication {
	private val promptInfo = BiometricPrompt.PromptInfo.Builder()
		.setTitle("UNLOCK SEED")
		.setSubtitle("Please authenticate yourself")
		.setAllowedAuthenticators(DEVICE_CREDENTIAL)
		.build()

	lateinit var biometricPrompt: BiometricPrompt
	lateinit var context: Context

	private fun getSecretKey(): SecretKey {
		val keyStore = KeyStore.getInstance("AndroidKeyStore")

		// Before the keystore can be accessed, it must be loaded.
		keyStore.load(null)
		return keyStore.getKey("test", null) as SecretKey
	}

	private fun getCipher(): Cipher {
		return Cipher.getInstance(
			KeyProperties.KEY_ALGORITHM_AES + "/"
			+ KeyProperties.BLOCK_MODE_CBC + "/"
			+ KeyProperties.ENCRYPTION_PADDING_PKCS7)
	}


	fun authenticate(activity: FragmentActivity) {
		val executor = ContextCompat.getMainExecutor(context)

		biometricPrompt = BiometricPrompt(
			activity, executor,
			object : BiometricPrompt.AuthenticationCallback() {
				override fun onAuthenticationError(errorCode: Int,
																					 errString: CharSequence) {
					super.onAuthenticationError(errorCode, errString)
					Toast.makeText(context,
						"Authentication error: $errString", Toast.LENGTH_SHORT)
						.show()
				}

				override fun onAuthenticationSucceeded(
					result: BiometricPrompt.AuthenticationResult) {
					super.onAuthenticationSucceeded(result)
					Toast.makeText(context,
						"Authentication succeeded!", Toast.LENGTH_SHORT)
						.show()
				}

				override fun onAuthenticationFailed() {
					super.onAuthenticationFailed()
					Toast.makeText(context, "Authentication failed",
						Toast.LENGTH_SHORT)
						.show()
				}
			})

		biometricPrompt.authenticate(promptInfo)//, BiometricPrompt.CryptoObject(cipher))
	}


}



