package io.parity.signer.screens.scan.transaction.dynamicderivations

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ExpandLess
import androidx.compose.material.icons.filled.ExpandMore
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetModel
import io.parity.signer.screens.keysetdetails.items.KeyDerivedItem
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textDisabled

@Composable
fun AddDerivedKeysScreen(
	model: AddDerivedKeysModel,
	onBack: Callback,
) {
	Column(
		modifier = Modifier
			.verticalScroll(rememberScrollState()),
	) {
		ScreenHeader(
			onBack = onBack,
			title = null,
			modifier = Modifier.padding(horizontal = 8.dp)
		)
		Text(
			text = "Add Derived Keys",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier.padding(horizontal = 24.dp),
		)
		Text(
			text = "Сheck the keys and scan QR code into Omni Wallet app",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
                .padding(horizontal = 24.dp)
                .padding(top = 8.dp, bottom = 20.dp),
		)

		//todo dmitry list of keysets

		Text(
			text = "Scan QR code to add the keys",//todo dmitry export text in this screen
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
                .padding(horizontal = 24.dp)
                .padding(top = 8.dp, bottom = 20.dp),
		)
		//todo dmitry qr code
		model.keysets.forEach { keyset ->

		}

		SecondaryButtonWide(
			label = stringResource(R.string.transaction_action_done),
			withBackground = true,
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 32.dp),
			onClicked = onBack,
		)
	}
}

@Composable
private fun KeysetItemDerivedItem(model: KeySetModel) {
	Column(
		modifier = Modifier
			.background(
				MaterialTheme.colors.fill6,
				RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
			)
			.animateContentSize()
	) {
		//network row
		Row(
			modifier = Modifier.clickable { collapsed.value = !collapsed.value },
			verticalAlignment = Alignment.CenterVertically
		) {

			NetworkIcon(
				networkLogoName = network.logo,
				modifier = Modifier
					.padding(
						top = 16.dp,
						bottom = 16.dp,
						start = 16.dp,
						end = 12.dp
					)
					.size(36.dp),
			)
			Text(
				text = network.title,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
			)
			Spacer(modifier = Modifier.weight(1f))
			Box(
				modifier = Modifier
					.padding(
						top = 20.dp,
						bottom = 20.dp,
						end = 16.dp,
						start = 12.dp
					)
					.background(MaterialTheme.colors.fill6, CircleShape),
				contentAlignment = Alignment.Center,
			) {
				Image(
					imageVector = if (collapsed.value) {
						Icons.Filled.ExpandMore
					} else {
						Icons.Filled.ExpandLess
					},
					modifier = Modifier
						.padding(4.dp)
						.size(24.dp),
					contentDescription = null,
					colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
				)
			}
		}
			var first = true
			keys.forEach { key ->
				SignerDivider(modifier = if (first) Modifier else Modifier.padding(start = 48.dp))
				KeyDerivedItem(model = key, network.logo, onClick = { onKeyClick(key, network) })
				first = false
			}
	}
}


data class AddDerivedKeysModel(val keysets: List<KeySetModel>) {
	companion object {
		fun createStub(): AddDerivedKeysModel = AddDerivedKeysModel(
			keysets = listOf(
				KeySetModel.createStub(),
				KeySetModel.createStub(name = "some2", number = 2)
			),
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
private fun PreviewAddDerivedKeysScreen() {


	SignerNewTheme {
		AddDerivedKeysScreen(
			model = AddDerivedKeysModel.createStub(),
			onBack = {},
		)
	}
}
