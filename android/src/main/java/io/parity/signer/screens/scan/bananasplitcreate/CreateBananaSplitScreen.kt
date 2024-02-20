package io.parity.signer.screens.scan.bananasplitcreate

import android.annotation.SuppressLint
import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
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
import androidx.compose.ui.res.stringResource
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
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.ui.theme.textTertiary


@Composable
fun CreateBananaSplitScreen(
	onClose: Callback,
	onCreate: Callback,
	updatePassowrd: (shards: Int) -> Unit,
	//todo dmitry remember password
	password: String,
	modifier: Modifier = Modifier,
) {
//todo dmitry implement https://www.figma.com/file/k0F8XYk9XVYdKLtkj0Vzp5/Signer-(Vault)-%C2%B7-Redesign?type=design&node-id=15728-46347&mode=design

	var shardsField: String = rememberSaveable { "4" }

	val shardsValue: Int? = shardsField.toIntOrNull()
	val canProceed: Boolean = shardsValue?.let { it > 2 } ?: false

	Column(modifier = modifier) {
		ScreenHeaderWithButton(
			canProceed = canProceed,
			btnText = stringResource(R.string.create_action_cta),
			onClose = onClose, onDone = onCreate
		)
		Column(Modifier.verticalScroll(rememberScrollState())) {
			Text(
				text = stringResource(R.string.create_bs_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
				modifier = Modifier
					.padding(horizontal = 24.dp)
			)
			Text(
				text = stringResource(R.string.create_bs_subtitle),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.LabelS,
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(vertical = 8.dp)
			)
			Text(
				text = stringResource(R.string.create_bs_shards_header),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier
					.padding(horizontal = 24.dp, vertical = 6.dp)
			)
			OutlinedTextField(
				value = shardsField,
				onValueChange = { newStr: String -> shardsField = newStr },
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
				isError = Integer.getInteger(shardsField) == null,
				singleLine = true,
				textStyle = SignerTypeface.LabelM,
				colors = TextFieldDefaults.textFieldColors(
					textColor = MaterialTheme.colors.primary,
					errorCursorColor = MaterialTheme.colors.primary,
				),
				modifier = Modifier
					.fillMaxWidth(1f)
					.padding(horizontal = 16.dp)
			)

			if (canProceed && shardsValue != null) {
				val requiresShards = BananaSplit.getMinShards(shardsValue)
				Text(
					text = stringResource(
						R.string.create_bs_shards_description_required,
						requiresShards,
						shardsValue
					),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.CaptionM,
					modifier = Modifier
						.padding(horizontal = 24.dp)
						.padding(vertical = 8.dp)
				)
			} else {
				//error description
				Text(
					text = stringResource(R.string.create_bs_shards_description_error),
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
					if (canProceed && shardsValue != null) {
						updatePassowrd(shardsValue)
					}
				},
			)

			Text(
				text = stringResource(R.string.create_bs_buttom_description),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.CaptionM,
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(vertical = 8.dp)
			)
			NotificationFrameTextImportant(
				message = stringResource(R.string.create_bs_notification_frame_text),
				modifier = Modifier.padding(horizontal = 16.dp, vertical = 32.dp),
				withBorder = false
			)
		}
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
			.padding(vertical = 8.dp, horizontal = 16.dp)
			.border(
				BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
				innerShape
			)

	) {

		Column(
			Modifier
				.weight(1f)
				.padding(vertical = 16.dp)
				.padding(start = 16.dp)
		) {
			Text(
				text = stringResource(R.string.create_bs_password_header),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyM,
				modifier = Modifier.padding(bottom = 4.dp)
			)
			Text(
				text = passPhrase,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier
			)
		}

		Icon(
			imageVector = Icons.Outlined.Refresh,
			contentDescription = stringResource(R.string.create_bs_refresh_password_icon_description),
			tint = MaterialTheme.colors.textSecondary,
			modifier = Modifier
				.align(Alignment.CenterVertically)
				.padding(start = 4.dp, end = 16.dp)
				.size(32.dp)
				.clickable(onClick = onUpdate)
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
