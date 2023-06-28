package io.parity.signer.screens.scan.transaction.dynamicderivations

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeyAndNetworkModel
import io.parity.signer.screens.keysetdetails.items.KeyDerivedItem
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6

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
			KeysetItemDerivedItem(keyset)
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
private fun KeysetItemDerivedItem(model: KeysetDerivedModel) {
	Column(
		modifier = Modifier
			.background(
				MaterialTheme.colors.fill6,
				RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
			)
			.animateContentSize()
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically
		) {
			Text(
				text = model.seedName,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				modifier = Modifier.padding(16.dp)
			)
			Spacer(modifier = Modifier.weight(1f))
		}
		//todo dmitry
		model.keysets.forEach { keyset ->
			SignerDivider(modifier = Modifier.padding(start = 48.dp))
			KeyDerivedItem(model = keyset.key, keyset.key.path, onClick = { })
		}
	}
}


data class AddDerivedKeysModel(
	val keysets: List<KeysetDerivedModel>,
) {
	companion object {
		fun createStub(): AddDerivedKeysModel = AddDerivedKeysModel(
			listOf(
				KeysetDerivedModel(
					seedName = "my special keyset",
					keysets = listOf(
						KeyAndNetworkModel.createStub(),
						KeyAndNetworkModel.createStub()
					),
				),
				KeysetDerivedModel(
					seedName = "my special keyset2",
					keysets = listOf(
						KeyAndNetworkModel.createStub(),
						KeyAndNetworkModel.createStub()
					),
				)
			)
		)
	}
}

data class KeysetDerivedModel(
	val seedName: String,
	val keysets: List<KeyAndNetworkModel>,
)


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
