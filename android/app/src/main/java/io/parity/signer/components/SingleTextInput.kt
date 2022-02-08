package io.parity.signer.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
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
import io.parity.signer.ui.theme.Border400
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600

@Composable
fun SingleTextInput(
	content: MutableState<String>,
	update: (String) -> Unit,
	onDone: () -> Unit,
	capitalize: Boolean = true,
	focusManager: FocusManager,
	focusRequester: FocusRequester
) {
	Surface(
		shape = MaterialTheme.shapes.medium,
		color = Color.Transparent,
		border = BorderStroke(1.dp, MaterialTheme.colors.Border400),
		modifier = Modifier.padding(20.dp)
	) {
		TextField(
			value = content.value,
			onValueChange = update,
			singleLine = true,
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
				textColor = MaterialTheme.colors.Text600,
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
