package io.parity.signer.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.TextField
import androidx.compose.material.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusManager
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.*

@Composable
fun SingleTextInput(
	content: MutableState<String>,
	update: (String) -> Unit,
	onDone: () -> Unit,
	capitalize: Boolean = true,
	prefix: (@Composable () -> Unit)? = null,
	isCrypto: Boolean = false,
	isCryptoColor: Boolean = false,
	focusManager: FocusManager,
	focusRequester: FocusRequester
) {
	Surface(
		shape = MaterialTheme.shapes.large,
		color = Color.Transparent,
		border = BorderStroke(1.dp, MaterialTheme.colors.Border400),
		modifier = Modifier.padding(vertical = 10.dp)
	) {
		TextField(
			value = content.value,
			onValueChange = update,
			singleLine = true,
			leadingIcon = prefix,
			textStyle = if (isCrypto) CryptoTypography.body2 else MaterialTheme.typography.body1,
			keyboardOptions = KeyboardOptions(
				autoCorrect = false,
				capitalization = if (capitalize) KeyboardCapitalization.Words else KeyboardCapitalization.None,
				keyboardType = KeyboardType.Password,
				imeAction = ImeAction.Done
			),
			keyboardActions = KeyboardActions(
				onDone = {
					onDone()
					focusManager.clearFocus()
				}
			),
			colors = TextFieldDefaults.textFieldColors(
				textColor = if (isCryptoColor) MaterialTheme.colors.Crypto400 else MaterialTheme.colors.Text600,
				backgroundColor = Color.Transparent,
				cursorColor = MaterialTheme.colors.Text400,
				leadingIconColor = MaterialTheme.colors.Text400,
				focusedIndicatorColor = Color.Transparent
			),
			modifier = Modifier
				.focusRequester(focusRequester = focusRequester)
				.fillMaxWidth(1f)
		)
	}
}
