package io.parity.signer.screens.keydetails

import android.content.res.Configuration
import androidx.annotation.DrawableRes
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.graphics.vector.rememberVectorPainter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.screens.keydetails.exportprivatekey.ConfirmExportPrivateKeyMenu
import io.parity.signer.components.base.RowButtonsBottomSheet
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.models.Callback
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.ui.theme.red400
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MKeyDetails

@Composable
fun KeyDetailsMenuAction(
	navigator: Navigator,
	keyDetails: MKeyDetails?
) {
	val state = remember {
		mutableStateOf(KeyDetailsMenuState.GENERAL)
	}
	when (state.value) {
		KeyDetailsMenuState.GENERAL -> KeyDetailsGeneralMenu(navigator, state)

		KeyDetailsMenuState.DELETE_CONFIRM -> KeyDetailsDeleteConfirmBottomSheet(
			onCancel = { navigator.backAction() },
			onRemoveKey = { navigator.navigate(Action.REMOVE_KEY) },
		)
		KeyDetailsMenuState.PRIVATE_KEY_CONFIRM -> ConfirmExportPrivateKeyMenu(
			navigator = navigator,
			publicKey = keyDetails!!.pubkey,
		)
	}
}

@Composable
private fun KeyDetailsGeneralMenu(
	navigator: Navigator,
	state: MutableState<KeyDetailsMenuState>
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 8.dp),
	) {

//		MenuItemForBottomSheet(
//			iconId = R.drawable.ic_networks_28,
//			label = stringResource(R.string.menu_option_address_on_other_networks),
//			tint = null,
//			onclick = {
//				//
//			}
//		)


		MenuItemForBottomSheet(
			iconId = R.drawable.ic_private_key_28,
			label = stringResource(R.string.menu_option_export_private_key),
			tint = null,
			onclick = {
				state.value = KeyDetailsMenuState.PRIVATE_KEY_CONFIRM
			}
		)

//		MenuItemForBottomSheet(
//			iconId = R.drawable.ic_blockchain_28,
//			label = stringResource(R.string.menu_option_change_keys_network),
//			tint = null,
//			onclick = {
//				//
//			}
//		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_backspace_28,
			label = stringResource(R.string.menu_option_forget_delete_key),
			tint = MaterialTheme.colors.red400,
			onclick = {
				state.value = KeyDetailsMenuState.DELETE_CONFIRM
			}
		)
		Spacer(modifier = Modifier.padding(bottom = 16.dp))
		SecondaryButtonBottomSheet(
			label = stringResource(R.string.generic_cancel),
		) {
			navigator.backAction()
		}
		Spacer(modifier = Modifier.padding(bottom = 16.dp))
	}
}


@Composable
fun KeyDetailsDeleteConfirmBottomSheet(
	onCancel: Callback,
	onRemoveKey: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 32.dp),
	) {

		Text(
			modifier = Modifier.fillMaxWidth(1f),
			text = stringResource(R.string.remove_key_confirm_title),
			color = MaterialTheme.colors.primary,
			style = TypefaceNew.TitleL,
			textAlign = TextAlign.Center,
		)
		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(
					top = 16.dp, bottom = 24.dp,
					start = 8.dp, end = 8.dp
				),
			text = stringResource(R.string.remove_key_confirm_text),
			color = MaterialTheme.colors.textSecondary,
			style = TypefaceNew.BodyL,
			textAlign = TextAlign.Center,
		)
		RowButtonsBottomSheet(
			labelCancel = stringResource(R.string.generic_cancel),
			labelCta = stringResource(R.string.remove_key_confirm_cta),
			onClickedCancel = onCancel,
			onClickedCta = onRemoveKey,
		)
		Spacer(modifier = Modifier.padding(bottom = 24.dp))

		DisposableEffect(Unit) {
			onDispose { onCancel() }
		}
	}
}


@Composable
internal fun MenuItemForBottomSheet(
	vector: ImageVector,
	label: String,
	tint: Color? = null,
	onclick: () -> Unit
) {
	MenuItemForBottomSheetInternal(
		onclick, rememberVectorPainter(vector),
		tint, label
	)
}

@Composable
internal fun MenuItemForBottomSheet(
	@DrawableRes iconId: Int,
	label: String,
	tint: Color? = null,
	onclick: () -> Unit
) {
	MenuItemForBottomSheetInternal(
		onclick, painterResource(id = iconId),
		tint, label
	)
}

@Composable
private fun MenuItemForBottomSheetInternal(
	onclick: () -> Unit,
	painter: Painter,
	tint: Color?,
	label: String
) {
	Row(
		modifier = Modifier
			.clickable(onClick = onclick)
			.padding(vertical = 8.dp)
			.fillMaxWidth(),
		verticalAlignment = Alignment.CenterVertically,
	) {
		Icon(
			painter = painter,
			contentDescription = null,
			modifier = Modifier
				.size(28.dp),
			tint = tint ?: MaterialTheme.colors.textSecondary,
		)
		Spacer(modifier = Modifier.padding(end = 24.dp))
		Text(
			text = label,
			color = tint ?: MaterialTheme.colors.textSecondary,
			style = TypefaceNew.TitleS,
		)
	}
}


private enum class KeyDetailsMenuState {
	GENERAL, DELETE_CONFIRM, PRIVATE_KEY_CONFIRM
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
private fun PreviewKeyDetailsGeneralMenu() {
	SignerNewTheme {
		KeyDetailsMenuAction(
			EmptyNavigator(),
			null
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
private fun PreviewKeyDetailsDeleteConfirmAction() {
	SignerNewTheme {
		KeyDetailsDeleteConfirmBottomSheet(
			{}, {},
		)
	}
}
