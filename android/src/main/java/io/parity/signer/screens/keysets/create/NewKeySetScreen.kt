package io.parity.signer.screens.keysets.create

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.components.base.DotsIndicator
import io.parity.signer.components.base.PrimaryButtonBottomSheet
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.uniffi.Action

/**
 * 1/2 stage to create new key set
 * second it NewKeySetBackup
 */
@Composable
fun NewKeySetScreen(
	rootNavigator: Navigator,
	seedNames: Array<String>
) {
	var keySetName by remember { mutableStateOf("") }
	val canProceed = keySetName.isNotEmpty() && !seedNames.contains(keySetName)
	Column(Modifier.background(MaterialTheme.colors.background)) {
		Row() {
			//close
			Spacer(modifier = Modifier.weight(1f))
			DotsIndicator(totalDots = 2, selectedIndex = 1)
			Spacer(modifier = Modifier.weight(1f))
			//next button
			PrimaryButtonBottomSheet(label = stringResource(R.string.button_next),
			isEnabled = canProceed,
			) {
				if (canProceed) rootNavigator.navigate(Action.GO_FORWARD, keySetName)
			}
		}

		Text(
			text = stringResource(R.string.new_key_set_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
		)
		Text(
			text = stringResource(R.string.new_key_set_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.LabelM,
		)
		//todo dmitry request focus
		TextField(
			value = "",
			onValueChange = { newStr -> keySetName = newStr },
//			keyboardActions = KeyboardActions(), todo dmitry
//			textStyle = LocalTextStyle.current,

		)
		Text(
			text = stringResource(R.string.new_key_set_description),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.CaptionM,
		)
	}
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewNewKeySetScreen() {
	SignerNewTheme {
		NewKeySetScreen(EmptyNavigator(), arrayOf())
	}
}
