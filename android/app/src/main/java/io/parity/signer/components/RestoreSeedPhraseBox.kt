package io.parity.signer.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.*
import org.json.JSONObject
import uniffi.signer.Action

@Composable
fun RestoreSeedPhraseBox(
	seedPhrase: List<JSONObject>,
	seedWord: TextFieldValue,
	button: (action: Action, details: String) -> Unit,
	keyboard: Boolean
) {
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Surface(
		border = BorderStroke(1.dp, MaterialTheme.colors.Border400),
		shape = MaterialTheme.shapes.large
	) {
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			//TODO: make this thing interactive
			Text(
				seedPhrase.sortedBy { it.optInt("order") }
					.joinToString(" ") { it.optString("content") },
				style = CryptoTypography.body1,
				color = MaterialTheme.colors.Crypto400,
				modifier = Modifier.fillMaxWidth().padding(12.dp)
			)
			Divider(color = MaterialTheme.colors.Border400)
			Row(horizontalArrangement = Arrangement.Center) {
				TextField(
					value = seedWord,
					onValueChange = {
						button(Action.TEXT_ENTRY, it.text)
					},
					singleLine = true,
					leadingIcon = {
						Text(
							">",
							style = MaterialTheme.typography.body2,
							color = MaterialTheme.colors.Text400						)
					},
					textStyle = CryptoTypography.body2,
					keyboardOptions = KeyboardOptions(
						autoCorrect = false,
						capitalization = KeyboardCapitalization.None,
						keyboardType = KeyboardType.Password,
						imeAction = ImeAction.Done
					),
					keyboardActions = KeyboardActions(
						onDone = {
							focusManager.clearFocus()
						}
					),
					colors = TextFieldDefaults.textFieldColors(
						textColor = MaterialTheme.colors.Text600,
						backgroundColor = Color.Transparent,
						cursorColor = MaterialTheme.colors.Text400,
						leadingIconColor = MaterialTheme.colors.Text400,
						focusedIndicatorColor = Color.Transparent
					),
					modifier = Modifier.focusRequester(focusRequester = focusRequester).fillMaxWidth(1f)
				)
			}
		}
	}

	DisposableEffect(Unit) {
		if (keyboard) {
			focusRequester.requestFocus()
		}
		onDispose { focusManager.clearFocus() }
	}
}
