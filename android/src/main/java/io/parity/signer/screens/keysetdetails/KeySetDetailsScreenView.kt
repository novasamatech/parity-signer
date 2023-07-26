package io.parity.signer.screens.keysetdetails

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.ChevronLeft
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.material.icons.filled.MoreHoriz
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.components.panels.BottomBar
import io.parity.signer.components.panels.BottomBarState
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.Callback
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.abbreviateString
import io.parity.signer.domain.conditional
import io.parity.signer.screens.keysetdetails.items.KeyDerivedItem
import io.parity.signer.screens.keysetdetails.items.SeedKeyDetails
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink300
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
	networkState: State<NetworkState?>, //for shield icon
	fullModelWasEmpty: Boolean,
	onFilterClicked: Callback,
	onMenu: Callback,
	onShowPublicKey: (title: String, key: String) -> Unit,
) {
	Column {
		KeySetDetailsHeader(
			onAddKey = {
				navigator.navigate(Action.NEW_KEY) //new derived key
			},
			onBack = { navigator.backAction() },
			onMenu = onMenu, //navigator.navigate(Action.RIGHT_BUTTON_ACTION) was in rust navigation
		)
		Box(modifier = Modifier.weight(1f)) {
			if (model.keysAndNetwork.isNotEmpty()) {
				Column(
					modifier = Modifier
						.verticalScroll(rememberScrollState()),
					verticalArrangement = Arrangement.spacedBy(4.dp),
				) {
					SeedKeyItemElement(model, onShowPublicKey)

					FilterRow(onFilterClicked)

					for (networkAndKeys in model.keysAndNetwork) {
						KeyDerivedItem(
							model = networkAndKeys.key,
							networkLogo = networkAndKeys.network.networkLogo,
						) {
							val selectKeyDetails =
								"${networkAndKeys.key.addressKey}\n${networkAndKeys.network.networkSpecsKey}"
							navigator.navigate(Action.SELECT_KEY, selectKeyDetails)
						}
					}
				}
			} else if (fullModelWasEmpty) {
				//no derived keys at all
				Column() {
					//seed
					SeedKeyItemElement(model, onShowPublicKey)
					KeySetDetailsEmptyList(onAdd = {
						navigator.navigate(Action.NEW_KEY, "") //new derived key
					})
				}
			} else {
				Column() {
					SeedKeyItemElement(model, onShowPublicKey)
					//no keys because filtered
					FilterRow(onFilterClicked)
					Spacer(modifier = Modifier.weight(0.5f))
					Text(
						text = stringResource(R.string.key_set_details_all_filtered_keys_title),
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.TitleM,
						textAlign = TextAlign.Center,
						modifier = Modifier.padding(horizontal = 40.dp)
					)
					Spacer(modifier = Modifier.weight(0.5f))
				}
			}

			ExposedIcon(
				networkState = networkState, navigator = navigator,
				Modifier
					.align(Alignment.BottomEnd)
					.padding(end = 16.dp, bottom = 24.dp)
			)
		}
		BottomBar(navigator, BottomBarState.KEYS)
	}
}

@Composable
private fun SeedKeyItemElement(model: KeySetDetailsModel,
															 onShowPublicKey: (title: String, key: String) -> Unit,
) {
	model.root?.let {
		SeedKeyDetails(
			model = it,
			onShowPublicKey = onShowPublicKey,
			Modifier
				.padding(horizontal = 24.dp, vertical = 8.dp)
				.padding(bottom = 16.dp)
		)
	}
}

@Composable
private fun FilterRow(onFilterClicked: Callback) {
	Row(
		modifier = Modifier.padding(horizontal = 24.dp, vertical = 8.dp),
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
				.clickable(onClick = onFilterClicked)
				.size(28.dp),
			tint = MaterialTheme.colors.pink300,
		)
	}
}


@Composable
fun KeySetDetailsHeader(
	onAddKey: Callback,
	onBack: Callback,
	onMenu: Callback,
) {
	Row(
		modifier = Modifier
			.fillMaxWidth(1f)
			.defaultMinSize(minHeight = 56.dp),
		verticalAlignment = Alignment.CenterVertically,
	) {
		Image(
			imageVector = Icons.Filled.ChevronLeft,
			contentDescription = stringResource(R.string.description_back_button),
			colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
			modifier = Modifier
				.padding(horizontal = 8.dp)
				.clickable(onClick = onBack)
				.padding(8.dp)
				.size(24.dp)
				.align(Alignment.CenterVertically)
		)
		//center
		Spacer(modifier = Modifier.weight(1f))
		//end
		Image(
			imageVector = Icons.Default.Add,
			contentDescription = stringResource(R.string.key_sets_details_screem_create_derived_button),
			colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
			modifier = Modifier
				.clickable(onClick = onAddKey)
				.padding(8.dp)
				.size(24.dp)
				.align(Alignment.CenterVertically)
		)
		Image(
			imageVector = Icons.Filled.MoreHoriz,
			contentDescription = stringResource(R.string.description_menu_button),
			colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
			modifier = Modifier
				.padding(end = 8.dp)
				.clickable(onClick = onMenu)
				.padding(8.dp)
				.size(24.dp)
				.align(Alignment.CenterVertically)
		)
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

@Composable
private fun KeySetDetailsEmptyList(onAdd: Callback) {
	Column(
		modifier = Modifier
			.fillMaxHeight(1f)
			.padding(horizontal = 16.dp),
		horizontalAlignment = Alignment.CenterHorizontally
	) {
		Spacer(modifier = Modifier.weight(0.5f))

		Column(
			modifier = Modifier
				.background(
					color = MaterialTheme.colors.pink300.copy(alpha = 0.12f),
					shape = RoundedCornerShape(dimensionResource(id = R.dimen.bigCornerRadius)),
				)
				.padding(24.dp)
		)
		{
			Text(
				text = stringResource(R.string.key_set_details_no_keys_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleM,
				textAlign = TextAlign.Center,
			)
			SecondaryButtonWide(
				label = stringResource(R.string.key_sets_details_screem_create_derived_button),
				withBackground = true,
				modifier = Modifier
					.padding(top = 16.dp, bottom = 24.dp, start = 24.dp, end = 24.dp),
				onClicked = onAdd
			)
		}
		Spacer(modifier = Modifier.weight(0.5f))
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
	val state = remember { mutableStateOf(NetworkState.Active) }
	val mockModel = KeySetDetailsModel.createStub()
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetDetailsScreenView(mockModel, EmptyNavigator(), state, false, {}, {}, {_,_ ->})
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
private fun PreviewKeySetDetailsScreenEmpty() {
	val state = remember { mutableStateOf(NetworkState.Active) }
	val mockModel =
		KeySetDetailsModel.createStub().copy(keysAndNetwork = emptyList())
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetDetailsScreenView(mockModel, EmptyNavigator(), state, true, {}, {}, {_,_ ->})
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
private fun PreviewKeySetDetailsScreenFiltered() {
	val state = remember { mutableStateOf(NetworkState.Active) }
	val mockModel =
		KeySetDetailsModel.createStub().copy(keysAndNetwork = emptyList())
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetDetailsScreenView(mockModel, EmptyNavigator(), state, false, {}, {}, {_,_ ->})
		}
	}
}
