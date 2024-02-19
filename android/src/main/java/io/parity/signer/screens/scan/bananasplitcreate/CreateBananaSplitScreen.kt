package io.parity.signer.screens.scan.bananasplitcreate

import android.annotation.SuppressLint
import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.material.TextFieldDefaults
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Refresh
import androidx.compose.runtime.Composable
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.NotificationFrameTextImportant
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.appliedStroke
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.ui.theme.textTertiary


@Composable
fun CreateBananaSplitScreen(
	onClose: Callback,
	onCreate: Callback,
	updatePassowrd: (shards: Int) -> Unit,
	password: String,
	modifier: Modifier = Modifier,
) {
//todo dmitry implement https://www.figma.com/file/k0F8XYk9XVYdKLtkj0Vzp5/Signer-(Vault)-%C2%B7-Redesign?type=design&node-id=15728-46347&mode=design

	var shards: String = rememberSaveable { "4" }

	val canProceed: Boolean = shards.toIntOrNull()?.let { it > 2 } ?: false

	Column(modifier = modifier) {
		ScreenHeaderWithButton(
			canProceed = canProceed,
			btnText = "Create",//todo dmitry export strings
			onClose = onClose, onDone = onCreate
		)
		Text(
			text = "Banana Split Backup",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
		)
		Text(
			text = "Backup your key set by turning the secret phrase into sharded QR codes with passphrase protection",
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(vertical = 8.dp)
		)
		Text(
			text = "Number of QR Code Shards",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp, vertical = 6.dp)
		)
		OutlinedTextField(
			value = shards,
			onValueChange = { newStr: String -> shards = newStr },
			visualTransformation = VisualTransformation.None,
			keyboardOptions = KeyboardOptions.Default.copy(
				keyboardType = KeyboardType.Number,
				//	fixme #1749 recreation of options leading to first letter dissapearing on some samsung devices so keeping it always Done
				imeAction = ImeAction.Done
			),
			keyboardActions = KeyboardActions(onDone = {
				if (canProceed) {
					onCreate()
				}
			}),
			isError = Integer.getInteger(shards) == null,
			singleLine = true,
			textStyle = SignerTypeface.LabelM,
			colors = TextFieldDefaults.textFieldColors(
				textColor = MaterialTheme.colors.primary,
				errorCursorColor = MaterialTheme.colors.primary,
			),
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 24.dp)
		)

		if (canProceed) {
			//todo how many shards
			Text(
				text = "3 shards out of 5 to reconstruct",//todo dmitry implement
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.CaptionM,
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(vertical = 8.dp)
			)
		} else {
			//error description
			Text(
				text = "The number of shares must be no less than 2 ",
				color = MaterialTheme.colors.error,
				style = SignerTypeface.CaptionM,
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(vertical = 8.dp)
			)
		}
		//passcode section
		PassPhraseBox(
			passPhrase = password,
			onUpdate = {
				updatePassowrd(4) //todo dmitry shards
			},
		)

		Text(
			text = "Write down your passphrase. You'll need it to recover from Banana Split.",
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(vertical = 8.dp)
		)
		NotificationFrameTextImportant(
			message = "Banana Split backup will recover the key set without derived keys. To back up derived keys, use the manual backup option. Each key will have to be added individually by entering the derivation path name.",
			modifier = Modifier.padding(horizontal = 8.dp, vertical = 10.dp),
			withBorder = false
		)
	}
}

@Composable
private fun PassPhraseBox(
	passPhrase: String,
	onUpdate: Callback,
	modifier: Modifier = Modifier
) {
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Row(
		modifier = modifier
			.padding(vertical = 8.dp)
			.border(
				BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
				innerShape
			)

	) {

		Column(Modifier.weight(1f)) {
			Text(
				text = "Passphrase for the Recovery",
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyM,
				modifier = Modifier
			)
			Text(
				text = passPhrase,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyM,
				modifier = Modifier
			)
		}

		Icon(
			imageVector = Icons.Outlined.Refresh,
			contentDescription = null,
			tint = MaterialTheme.colors.textSecondary,
			modifier = Modifier
				.align(Alignment.CenterVertically)
				.padding(start = 18.dp, end = 18.dp)
		)
	}
}

@SuppressLint("UnrememberedMutableState")
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
private fun PreviewCreateBananaSplitScreen() {
	SignerNewTheme {
		CreateBananaSplitScreen(
			onClose = {},
			onCreate = {},
			updatePassowrd = {},
			password = "delirium-claim-clad-down"
		)
	}
}
