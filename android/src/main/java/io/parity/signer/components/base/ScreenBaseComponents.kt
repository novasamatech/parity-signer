package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBackIos
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.HelpOutline
import androidx.compose.material.icons.filled.MoreHoriz
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary


@Composable
fun ScreenHeader(
	title: String?,
	onBack: Callback? = null,
	onMenu: Callback? = null,
) {
	Row(
		modifier = Modifier
			.fillMaxWidth(1f)
			.defaultMinSize(minHeight = 56.dp)
	) {
		if (onBack != null) {
			Image(
				imageVector = Icons.Filled.ArrowBackIos,
				contentDescription = stringResource(R.string.description_back_button),
				colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
				modifier = Modifier
					.padding(horizontal = 8.dp)
					.clickable(onClick = onBack)
					.padding(8.dp)
					.size(24.dp)
					.align(Alignment.CenterVertically)
			)
		} else {
			Spacer(modifier = Modifier.padding(start = 56.dp))
		}
		//center
		if (title != null) {
			Text(
				text = title,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				textAlign = TextAlign.Center,
				modifier = Modifier
					.align(Alignment.CenterVertically)
					.weight(1f)
			)
		} else {
			Spacer(modifier = Modifier.weight(1f))
		}
		//end
		if (onMenu != null) {
			Image(
				imageVector = Icons.Filled.MoreHoriz,
				contentDescription = stringResource(R.string.description_menu_button),
				colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
				modifier = Modifier
					.padding(horizontal = 8.dp)
					.clickable(onClick = onMenu)
					.padding(8.dp)
					.size(24.dp)
					.align(Alignment.CenterVertically)
			)
		} else {
			Spacer(modifier = Modifier.padding(start = 56.dp))
		}
	}
}

@Composable
fun ScreenHeaderClose(
	title: String,
	subtitle: String? = null,
	onClose: Callback,
	onMenu: Callback? = null,
	differentMenuIcon: ImageVector? = null,
) {
	Row(
		modifier = Modifier
			.fillMaxWidth(1f)
			.defaultMinSize(minHeight = 56.dp)
	) {
		Image(
			imageVector = Icons.Filled.Close,
			contentDescription = stringResource(R.string.description_back_button),
			colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
			modifier = Modifier
				.padding(horizontal = 8.dp)
				.clickable(onClick = onClose)
				.padding(8.dp)
				.size(24.dp)
				.align(Alignment.CenterVertically)
		)
		//center
		Column(
			modifier = Modifier
				.align(Alignment.CenterVertically)
				.weight(1f)
		) {
			Text(
				text = title,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				textAlign = TextAlign.Center,
				modifier = Modifier.fillMaxWidth(1f),
			)
			if (subtitle != null) {
				Text(
					text = subtitle,
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.CaptionM,
					textAlign = TextAlign.Center,
					modifier = Modifier.fillMaxWidth(1f),
				)
			}
		}
		//end
		if (onMenu != null) {
			Image(
				imageVector = differentMenuIcon ?: Icons.Filled.MoreHoriz,
				contentDescription = stringResource(R.string.description_menu_button),
				colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
				modifier = Modifier
					.padding(horizontal = 8.dp)
					.clickable(onClick = onMenu)
					.padding(8.dp)
					.size(24.dp)
					.align(Alignment.CenterVertically)
			)
		} else {
			Spacer(modifier = Modifier.padding(start = 56.dp))
		}
	}
}


@Composable
fun ScreenHeaderWithButton(
	canProceed: Boolean,
	title: String = "",
	btnText: String? = null,
	backNotClose: Boolean = false,
	modifier: Modifier = Modifier,
	onClose: Callback,
	onDone: Callback,
) {
	Row(
		modifier = modifier.padding(
			start = 8.dp,
			end = 8.dp,
			top = 8.dp,
			bottom = 8.dp
		),
		verticalAlignment = Alignment.CenterVertically,
	) {
		if (backNotClose) {
			Image(
				imageVector = Icons.Filled.ArrowBackIos,
				contentDescription = stringResource(R.string.description_back_button),
				colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
				modifier = Modifier
					.padding(end = 8.dp)
					.clickable(onClick = onClose)
					.padding(8.dp)
					.size(24.dp)
			)
		} else {
			Image(
				imageVector = Icons.Filled.Close,
				contentDescription = stringResource(R.string.description_back_button),
				colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
				modifier = Modifier
					.padding(end = 8.dp)
					.clickable(onClick = onClose)
					.padding(8.dp)
					.size(24.dp)
					.align(Alignment.CenterVertically)
			)
		}
		Box(
			modifier = Modifier.weight(1f),
			contentAlignment = Alignment.Center,
		) {
			Text(
				text = title,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				textAlign = TextAlign.Center,
				modifier = Modifier.fillMaxWidth(1f)
			)
		}
		Box(
			contentAlignment = Alignment.CenterEnd,
		) {
			PrimaryButtonGreyDisabled(
				label = btnText ?: stringResource(R.string.generic_done),
				isEnabled = canProceed,
			) {
				if (canProceed) {
					onDone()
				}
			}
		}
	}
}


@Preview(
	name = "day",
	group = "themes",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
	backgroundColor = 0xFFFFFFFF
)
@Preview(
	name = "dark theme",
	group = "themes",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true,
	backgroundColor = 0xFF000000
)
@Composable
private fun PreviewScreenBaseComponent() {
	SignerNewTheme() {
		Column(
			modifier = Modifier.size(500.dp),
		) {
			ScreenHeader(
				null,
				onBack = {},
				onMenu = {},
			)
			ScreenHeader(
				stringResource(R.string.key_sets_screem_title),
				onBack = null,
				onMenu = {},
			)
			ScreenHeader(
				stringResource(id = R.string.key_sets_screem_title),
				onBack = null,
				onMenu = null,
			)
			ScreenHeaderClose(
				stringResource(id = R.string.key_sets_screem_title),
				onClose = {},
				onMenu = null,
			)
			ScreenHeaderClose(
				stringResource(id = R.string.key_sets_screem_title),
				"subtitle",
				onClose = {},
				onMenu = {},
			)
			ScreenHeaderWithButton(true, "Derivation", null, true, Modifier, {}, {})
			ScreenHeaderWithButton(true, "Derivation", null, false, Modifier, {}, {})
			ScreenHeaderWithButton(false, "Derivation", null, false, Modifier, {}, {})
			ScreenHeaderClose(
				stringResource(id = R.string.key_sets_screem_title),
				onClose = {},
				onMenu = {},
				differentMenuIcon = Icons.Filled.HelpOutline,
			)
		}
	}
}
