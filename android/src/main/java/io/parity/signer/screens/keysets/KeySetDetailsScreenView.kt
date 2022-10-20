package io.parity.signer.screens.keysets

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonBottomSheet
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.components.items.KeyDerivedItem
import io.parity.signer.components.items.KeySetItem
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.models.*
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.*

//todo old screen is KeyManager
/**
 * Single Seed/Key set is selected is it's details
 * For non-multiselect state
 */
@Composable
fun KeySetDetailsScreenView(
	model: KeySetDetailsModel,
	navigator: Navigator,
	alertState: State<AlertState?>, //for shield icon
) {

	Column {
		ScreenHeader(
			stringId = null,
			onback = { navigator.backAction() },
			onMenu = { navigator.navigate(Action.RIGHT_BUTTON_ACTION) }
		)
		Box(modifier = Modifier.weight(1f)) {
			Column(
				modifier = Modifier.verticalScroll(rememberScrollState())
			) {
				//seed
				SeedKeyViewItem(model.root) //todo dmitry on click
				//filter row
				Row(
					modifier = Modifier.padding(horizontal = 24.dp),
					verticalAlignment = Alignment.CenterVertically
				) {
					Text(
						text = "Derived Keys",
						color = MaterialTheme.colors.textTertiary,
						style = TypefaceNew.BodyM,
						modifier = Modifier.weight(1f),
					)
					Icon(
						painter = painterResource(id = R.drawable.ic_tune_28),
						contentDescription = null,
						modifier = Modifier
							.size(28.dp),//todo dmitry on click
						tint = MaterialTheme.colors.textTertiary,
					)
				}
				for(key in model.keys) {
					KeyDerivedItem(model = key) {
						//todo dmitry on click
					}
				}
			}

			Column(modifier = Modifier.align(Alignment.BottomCenter)) {
				ExposedIcon(
					alertState = alertState, navigator = navigator,
					Modifier
						.align(Alignment.End)
						.padding(end = 16.dp)
				)
				PrimaryButtonBottomSheet(
					label = stringResource(R.string.key_sets_screem_add_key_button), //todo dmitry new derived key
					modifier = Modifier
						.padding(top = 16.dp, bottom = 24.dp, start = 24.dp, end = 24.dp)
				) {
					navigator.navigate(Action.RIGHT_BUTTON_ACTION) //todo dmitry new derived key
				}
			}
		}
		BottomBar2(navigator, BottomBar2State.KEYS)
	}
}

@Composable
private fun SeedKeyViewItem(seedKeyModel: SeedKeyModel) {
	Row(
		modifier = Modifier.padding(top = 16.dp, bottom = 16.dp, start = 24.dp),
		verticalAlignment = Alignment.CenterVertically,
	) {
		Column(Modifier.weight(1f)) {
			Text(
				text = seedKeyModel.seedName,
				color = MaterialTheme.colors.primary,
				style = TypefaceNew.TitleL,
			)
			Text(
				text = seedKeyModel.base58.abbreviateString(8),
				color = MaterialTheme.colors.textTertiary,
				style = TypefaceNew.BodyM,
			)
		}
		Image(
			imageVector = Icons.Filled.ChevronRight,
			contentDescription = null,
			colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
			modifier = Modifier
				.padding(end = 16.dp)
				.size(28.dp)
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
private fun PreviewKeySetDetailsScreen() {

	val state = remember { mutableStateOf(AlertState.Active) }
	val mockModel = KeySetDetailsModel.createStub()
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetDetailsScreenView(mockModel, EmptyNavigator(), state)
		}
	}
}
