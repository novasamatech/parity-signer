package io.parity.signer.domain

import android.app.KeyguardManager
import android.content.Context
import android.widget.Toast
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import io.parity.signer.R
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlin.coroutines.suspendCoroutine


class Authentication {

	companion object {
		fun canAuthenticate(context: Context): Boolean {
			val keygoard =
				context.getSystemService(Context.KEYGUARD_SERVICE) as KeyguardManager
			return keygoard.isDeviceSecure
		}
	}

	/**
	 * Due to BiometricPrompt.java:553 [setAllowedAuthenticators()] description we can't always require only
	 * Device credentials as have to allow biometric on older android versions
	 *
	 * BiometricManager.Authenticators.DEVICE_CREDENTIAL should be used above 31
	 */
	private fun getRequiredAuthMethods(): Int {
		return when (android.os.Build.VERSION.SDK_INT) {
			in 30..Int.MAX_VALUE -> {
				BiometricManager.Authenticators.DEVICE_CREDENTIAL
			}
			28, 29 -> {
				BiometricManager.Authenticators.DEVICE_CREDENTIAL or BiometricManager.Authenticators.BIOMETRIC_WEAK
			}
			else -> {
				BiometricManager.Authenticators.DEVICE_CREDENTIAL or BiometricManager.Authenticators.BIOMETRIC_STRONG
			}
		}
	}

	private lateinit var biometricPrompt: BiometricPrompt

	private val _auth = MutableStateFlow<Boolean>(false)
	val auth: StateFlow<Boolean> = _auth

	fun authenticate(activity: FragmentActivity, onSuccess: () -> Unit) {
		if (FeatureFlags.isEnabled(FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT)) {
			_auth.value = true
			onSuccess()
			return
		}

		val context = activity.baseContext

		val promptInfo =
			BiometricPrompt.PromptInfo.Builder()
				.setTitle(context.getString(R.string.unlock_device_title))
				.setSubtitle(context.getString(R.string.unlock_device_subtitle))
				.setAllowedAuthenticators(getRequiredAuthMethods())
				.build()

		if (canAuthenticate(context)) {

			val executor = ContextCompat.getMainExecutor(context)

			biometricPrompt = BiometricPrompt(
				activity, executor,
				object : BiometricPrompt.AuthenticationCallback() {
					override fun onAuthenticationError(
						errorCode: Int,
						errString: CharSequence
					) {
						super.onAuthenticationError(errorCode, errString)
						Toast.makeText(
							context,
							context.getString(R.string.auth_error_message, errString),
							Toast.LENGTH_SHORT
						).show()
						_auth.value = false
					}

					override fun onAuthenticationSucceeded(
						result: BiometricPrompt.AuthenticationResult
					) {
						super.onAuthenticationSucceeded(result)
						_auth.value = true
						onSuccess()
					}

					override fun onAuthenticationFailed() {
						super.onAuthenticationFailed()
						Toast.makeText(
							context, context.getString(R.string.auth_failed_message),
							Toast.LENGTH_SHORT
						).show()
					}
				})

			biometricPrompt.authenticate(promptInfo)
		} else {
			Toast.makeText(
				context, context.getString(R.string.auth_error_status_unknown),
				Toast.LENGTH_LONG
			).show()
		}
	}

	suspend fun authenticate(activity: FragmentActivity): AuthResult =
		suspendCoroutine { continuation ->
			val context = activity.baseContext

			val promptInfo =
				BiometricPrompt.PromptInfo.Builder()
					.setTitle(context.getString(R.string.unlock_device_title))
					.setSubtitle(context.getString(R.string.unlock_device_subtitle))
					.setAllowedAuthenticators(getRequiredAuthMethods())
					.build()

			if (canAuthenticate(context)) {

				val executor = ContextCompat.getMainExecutor(context)

				biometricPrompt = BiometricPrompt(
					activity, executor,
					object : BiometricPrompt.AuthenticationCallback() {
						override fun onAuthenticationError(
							errorCode: Int,
							errString: CharSequence
						) {
							super.onAuthenticationError(errorCode, errString)
							Toast.makeText(
								context,
								context.getString(R.string.auth_error_message, errString),
								Toast.LENGTH_SHORT
							).show()
							_auth.value = false
							continuation.resumeWith(Result.success(AuthResult.AuthError))
						}

						override fun onAuthenticationSucceeded(
							result: BiometricPrompt.AuthenticationResult
						) {
							super.onAuthenticationSucceeded(result)
							_auth.value = true
							continuation.resumeWith(Result.success(AuthResult.AuthSuccess))
						}

						override fun onAuthenticationFailed() {
							super.onAuthenticationFailed()
							Toast.makeText(
								context, context.getString(R.string.auth_failed_message),
								Toast.LENGTH_SHORT
							).show()
						}
					}
				)
				biometricPrompt.authenticate(promptInfo)
			} else {
				Toast.makeText(
					context, context.getString(R.string.auth_error_status_unknown),
					Toast.LENGTH_LONG
				).show()
			}
		}
}


	sealed class AuthResult {
		data object AuthSuccess : AuthResult()
		data object AuthError : AuthResult()
		data object AuthFailed : AuthResult()
		data object AuthUnavailable : AuthResult()
	}


