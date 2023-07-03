package io.parity.signer.screens.createderivation

import android.content.Context
import androidx.compose.material.Colors
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.input.OffsetMapping
import androidx.compose.ui.text.input.TransformedText
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.text.withStyle
import io.parity.signer.R
import io.parity.signer.ui.theme.textTertiary
import kotlin.math.min


class DerivationPathAnalyzer {
	// stolen from rust/db_handling/src/identities.rs:99 where it was stolen from sp_core
	private val regexCheckPath: Regex = "^((//?[^/]+)*)(///.*)?$".toRegex()

	fun isCorrect(path: String): Boolean {
		return path.isEmpty() || regexCheckPath.matches(path)
	}

	fun getHint(path: String): Hint {
		return when {
			path.isEmpty() -> Hint.CREATING_ROOT
			getPassword(path) == "" -> Hint.CREATE_PASSWORD
			path.endsWith("//") && getPassword(path) == null -> Hint.PATH_NAME
			else -> Hint.NONE
		}
	}

	enum class Hint {
		PATH_NAME, CREATE_PASSWORD, CREATING_ROOT, NONE;

		fun getLocalizedString(context: Context): String? {
			return when (this) {
				PATH_NAME -> context.getString(R.string.derivation_path_hint_enter_path_name)
				CREATE_PASSWORD -> context.getString(R.string.derivation_path_hint_create_password)
				CREATING_ROOT -> context.getString(R.string.derivation_path_hint_it_is_root)
				NONE -> null
			}
		}
	}

	companion object {
		fun getPassword(path: String): String? {
			return if (path.contains("///")) {
				path.substringAfter("///")
			} else {
				null
			}
		}

		fun hidePasswords(text: String): String {
			val maskStar: String = '\u2022'.toString()
			val password = getPassword(text)
			return if (password == null) {
				text
			} else {
				text.replaceAfter("///", maskStar.repeat(password.length))
			}
		}
	}
}


data class DerivationPathVisualTransformation(
	val context: Context,
	val themeColors: Colors,
	val hidePassword: Boolean,
) : VisualTransformation {

	val pathAnalyzer = DerivationPathAnalyzer()

	override fun filter(text: AnnotatedString): TransformedText {
		val content = buildAnnotatedString {
			if (hidePassword) {
				append(DerivationPathAnalyzer.hidePasswords(text.text))
			} else {
				append(text.text)
			}
			pathAnalyzer.getHint(text.text).getLocalizedString(context)
				?.let { hint ->
					append(" ")
					withStyle(SpanStyle(color = themeColors.textTertiary)) {
						append(hint)
					}
				}
		}
		return TransformedText(content, DerivationOffsetMapping(text.length))
	}

	/**
	 * We append hint to original but transformed cannot be smaller than original
	 */
	class DerivationOffsetMapping(private val originalSize: Int) : OffsetMapping {
		override fun originalToTransformed(offset: Int): Int {
			return offset
		}

		override fun transformedToOriginal(offset: Int): Int {
			return min(originalSize, offset)
		}
	}
}

