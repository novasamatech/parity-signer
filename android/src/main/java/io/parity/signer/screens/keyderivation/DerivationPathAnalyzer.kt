package io.parity.signer.screens.keyderivation

import android.content.Context
import androidx.compose.material.Colors
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.input.OffsetMapping
import androidx.compose.ui.text.input.TransformedText
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.text.withStyle
import io.parity.signer.ui.theme.textTertiary
import kotlin.math.min


class DerivationPathAnalyzer {
	// stolen from rust/db_handling/src/identities.rs:99 where it was stolen from sp_core //todo
	val REG_PATH: Regex = "^(//?[^/]*)(///+)?$".toRegex()

	//todo rust/db_handling/src/identities.rs:1123 - validation
	fun isCorrect(path: String): Boolean {
		return REG_PATH.matches(path)
	}

	fun getPassword(path: String): String? {
		return path.split("///").lastOrNull()
	}

	fun getHint(path: String): Hint {
		return when {
			path.endsWith("///") -> Hint.CREATE_PASSWORD
			path.endsWith("//") -> Hint.PATH_NAME
			else -> Hint.NONE
		}
	}

	enum class Hint {
		PATH_NAME, CREATE_PASSWORD, NONE;

		fun getLocalizedString(context: Context): String? {
			return when (this) {
				PATH_NAME -> "EnterPathName"
				CREATE_PASSWORD -> "Create a Password"
				NONE -> null
			}
		}
	}
}


//todo derivation what to show what to show if wrong path
//PasswordVisualTransformation
class DerivationPathVisualTransformation(
	val context: Context,
	val themeColors: Colors
) :
	VisualTransformation {

	val pathAnalyzer = DerivationPathAnalyzer()

	override fun filter(text: AnnotatedString): TransformedText {
		val content = if (pathAnalyzer.isCorrect(text.text)) {
			buildAnnotatedString {
				append(hidePasswords(text.text))
				pathAnalyzer.getHint(text.text).getLocalizedString(context)
					?.let { hint ->
						append(" ")
						withStyle(SpanStyle(color = themeColors.textTertiary)) {
							append(hint)
						}
					}
			}
		} else {
			text
		}
		return TransformedText(content, DerivationOffsetMapping(text.length))
	}

	private fun hidePasswords(text: String): String {
		val maskStar: String = '\u2022'.toString()
		var newText = text
		val passwords = text.split("///").drop(1)
		passwords.forEach { pass ->
			newText = newText.replace(pass, maskStar.repeat(pass.length))
		}
		return newText
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

