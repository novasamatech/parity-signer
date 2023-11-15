package io.parity.signer.screens.createderivation.derivationsubscreens

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CheckboxWithText
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*


@Composable
fun DerivationCreateConfirmBottomSheet(
	path: String,
	onDone: Callback,
) {
	var checkboxConfirmed by remember { mutableStateOf(false) }

	Column(
		Modifier.padding(horizontal = 24.dp, vertical = 24.dp)
			.imePadding(),
	) {

		Text(
			text = stringResource(R.string.new_derivation_conformation_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
		)
		Text(
			text = stringResource(R.string.new_derivation_conformation_message),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyM,
			modifier = Modifier.padding(top = 12.dp, bottom = 24.dp),
		)
		val innerShape =
			RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))

		Column(
			modifier = Modifier
				.border(
					BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
					innerShape
				)
				.background(MaterialTheme.colors.fill6, innerShape)
				.padding(16.dp)
				.fillMaxWidth(1f)
		) {
			Text(
				text = stringResource(R.string.new_derivation_conformation_path_headline),
				color = MaterialTheme.colors.textSecondary,
				style = SignerTypeface.BodyM,
			)
			Spacer(modifier = Modifier.padding(top = 6.dp))
			Text(
				text = path,
				color = MaterialTheme.colors.pink300,
				style = SignerTypeface.BodyL,
			)
		}
		CheckboxWithText(
			checked = checkboxConfirmed,
			text = stringResource(R.string.new_derivation_conformation_checkbox_wrote_down),
			modifier = Modifier.padding(top = 16.dp, bottom = 24.dp),
		) { newValue ->
			checkboxConfirmed = newValue
		}
		PrimaryButtonWide(
			label = stringResource(id = R.string.generic_done),
			isEnabled = checkboxConfirmed,
			onClicked = onDone,
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
private fun PreviewDerivationCreateConfirmBottomSheet() {
	SignerNewTheme {
		DerivationCreateConfirmBottomSheet("//polkadot//1///pass", {})
	}
}
