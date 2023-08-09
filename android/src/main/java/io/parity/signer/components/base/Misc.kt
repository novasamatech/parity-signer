package io.parity.signer.components.base

import androidx.annotation.StringRes
import androidx.compose.foundation.clickable
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textDisabled


@Composable
fun ClickableLabel(
	@StringRes stringId: Int,
	isEnabled: Boolean,
	modifier: Modifier = Modifier,
	onClick: () -> Unit,
) {
	val modifier = if (isEnabled) {
		modifier.clickable(onClick = onClick)
	} else {
		modifier
	}
	Text(
		text = stringResource(id = stringId),
		color = if (isEnabled) {
			MaterialTheme.colors.pink300
		} else {
			MaterialTheme.colors.textDisabled
		},
		style = SignerTypeface.TitleS,
		modifier = modifier,
	)
}
