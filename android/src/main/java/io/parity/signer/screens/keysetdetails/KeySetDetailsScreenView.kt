package io.parity.signer.screens.keysetdetails

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
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
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.models.*
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.Action

/**
 * Single Seed/Key set is selected is it's details
 * For non-multiselect state,
 * For multiselec screen KeyManager is still used
 */
@Composable
fun KeySetDetailsScreenView(
	model: KeySetDetailsModel,
	navigator: Navigator,
	alertState: State<AlertState?>, //for shield icon
	onMenu: Callback,
) {
	Column {
		ScreenHeader(
			stringId = null,
			onBack = { navigator.backAction() },
			onMenu = onMenu, //navigator.navigate(Action.RIGHT_BUTTON_ACTION) was in rust navigation
		)
		Box(modifier = Modifier.weight(1f)) {
			Column(
				modifier = Modifier.verticalScroll(rememberScrollState())
			) {
				//seed
				SeedKeyViewItem(model.root) {
					navigator.navigate(Action.SELECT_KEY, model.root.addressKey)
				}
				//filter row
				Row(
					modifier = Modifier.padding(horizontal = 24.dp),
					verticalAlignment = Alignment.CenterVertically
				) {
					Text(
						text = stringResource(R.string.key_sets_details_screem_derived_subtitle),
						color = MaterialTheme.colors.textTertiary,
						style = SignerTypeface.BodyM,
						modifier = Modifier.weight(1f),
					)
					Icon(
						painter = painterResource(id = R.drawable.ic_tune_28),
						contentDescription = stringResource(R.string.key_sets_details_screem_filter_icon_description),
						modifier = Modifier
                            .clickable {
                                navigator.navigate(
                                    Action.NETWORK_SELECTOR,
                                    ""
                                )
                            }
                            .size(28.dp),
						tint = MaterialTheme.colors.textTertiary,
					)
				}
				for (key in model.keys) {
					KeyDerivedItem(model = key) {
						navigator.navigate(Action.SELECT_KEY, key.addressKey)
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
					label = stringResource(R.string.key_sets_details_screem_create_derived_button),
					modifier = Modifier
						.padding(top = 16.dp, bottom = 24.dp, start = 24.dp, end = 24.dp)
				) {
					navigator.navigate(Action.NEW_KEY, "") //new derived key
				}
			}
		}
		BottomBar2(navigator, BottomBar2State.KEYS)
	}
}

/**
 * Not clickable item - disabled automatically
 */
@Composable
fun SeedKeyViewItem(
	seedKeyModel: KeyModel,
	onClick: Callback?,
) {
	Surface(
		modifier = Modifier
			.conditional(onClick != null) {
				clickable(onClick = onClick!!)
			},
		color = Color.Transparent,
	)
	{
		Row(
			modifier = Modifier
				.padding(top = 16.dp, bottom = 16.dp, start = 24.dp),
			verticalAlignment = Alignment.CenterVertically,
		) {
			Column(Modifier.weight(1f)) {
				Text(
					text = seedKeyModel.seedName,
					color = if (onClick != null) MaterialTheme.colors.primary else MaterialTheme.colors.textDisabled,
					style = SignerTypeface.TitleL,
				)
				Text(
					text = seedKeyModel.base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
					color = if (onClick != null) MaterialTheme.colors.textTertiary else MaterialTheme.colors.textDisabled,
					style = SignerTypeface.BodyM,
				)
			}
			if (onClick != null) {
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
			KeySetDetailsScreenView(mockModel, EmptyNavigator(), state, {})
		}
	}
}
