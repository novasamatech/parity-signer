package io.parity.signer.screens.keydetails

import android.content.res.Configuration
import androidx.annotation.DrawableRes
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.graphics.vector.rememberVectorPainter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetConfirmDialog
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.red400
import io.parity.signer.ui.theme.textSecondary

@Composable
internal fun KeyDetailsGeneralMenu(
	closeMenu: Callback,
	onExportPrivateKey: Callback,
	onDelete: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 8.dp),
	) {

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_private_key_28,
			label = stringResource(R.string.menu_option_export_private_key),
			tint = null,
			onclick = onExportPrivateKey,
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_backspace_28,
			label = stringResource(R.string.menu_option_forget_delete_key),
			tint = MaterialTheme.colors.red400,
			onclick = onDelete,
		)
		Spacer(modifier = Modifier.padding(bottom = 8.dp))
		SecondaryButtonWide(
			label = stringResource(R.string.generic_cancel),
			onClicked = closeMenu,
		)
		Spacer(modifier = Modifier.padding(bottom = 16.dp))
	}
}


@Composable
fun KeyDetailsDeleteConfirmBottomSheet(
	onCancel: Callback,
	onRemoveKey: Callback,
) {
	BottomSheetConfirmDialog(
		title = stringResource(R.string.remove_key_confirm_title),
		message = stringResource(R.string.remove_key_confirm_text),
		ctaLabel = stringResource(R.string.remove_key_confirm_cta),
		onCancel = onCancel,
		onCta = onRemoveKey,
	)
}


@Composable
internal fun MenuItemForBottomSheet(
	vector: ImageVector,
	label: String,
	tint: Color? = null,
	onclick: () -> Unit,
	modifier: Modifier = Modifier,
) {
	MenuItemForBottomSheetInternal(
		onclick, rememberVectorPainter(vector),
		tint, label, modifier = modifier,
	)
}

@Composable
internal fun MenuItemForBottomSheet(
	@DrawableRes iconId: Int,
	label: String,
	tint: Color? = null,
	onclick: () -> Unit,
	modifier: Modifier = Modifier,
) {
	MenuItemForBottomSheetInternal(
		onclick = onclick, painter = painterResource(id = iconId),
		tint = tint, label = label, modifier = modifier,
	)
}

@Composable
private fun MenuItemForBottomSheetInternal(
	onclick: () -> Unit,
	painter: Painter,
	tint: Color?,
	label: String,
	modifier: Modifier = Modifier,
) {
	Row(
		modifier = modifier
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
			style = SignerTypeface.TitleS,
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
private fun PreviewKeyDetailsGeneralMenu() {
	SignerNewTheme {
		KeyDetailsGeneralMenu(
			{}, {}, {},
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
