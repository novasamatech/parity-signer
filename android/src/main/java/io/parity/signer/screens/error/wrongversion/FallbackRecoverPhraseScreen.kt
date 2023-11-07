package io.parity.signer.screens.error.wrongversion

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
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
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textDisabled

@Composable
fun FallbackRecoverPhraseScreen(
	seedList: List<String>,
	onSelected: (clickedKeyName: String) -> Unit,
	onBack: Callback,
) {
	Column {

		ScreenHeader(
			title = stringResource(R.string.fallback_recovery_phrase_title),
			onBack = onBack
		)
		Column(Modifier.verticalScroll(rememberScrollState())) {
			Text(
				modifier = Modifier
					.fillMaxWidth(1f)
					.padding(vertical = 8.dp, horizontal = 24.dp),
				text = stringResource(R.string.fallback_recovery_phrase_description),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
			)
			Column(
				modifier = Modifier
					.padding(8.dp)
					.background(
						MaterialTheme.colors.fill6,
						RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
					)
			) {
				seedList.forEachIndexed { index, it ->
					KeysetEntry(it) {
						onSelected(it)
					}
					if (index < seedList.lastIndex) {
						SignerDivider()
					}
				}
			}
		}
	}
}

@Composable
private fun KeysetEntry(name: String, onClicked: Callback) {
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier.clickable(onClick = onClicked),
	) {
		Text(
			text = name,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.LabelM,
			modifier = Modifier
				.weight(1f)
				.padding(16.dp),
		)
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
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun FallbackRecoverPhraseScreenPreview() {
	SignerNewTheme() {
		FallbackRecoverPhraseScreen(
			seedList = listOf(
				"Omni Wallet", "One very very very very very very long lina name text",
				"Stacking", "Nova Wallet", "Crowdloans"
			),
			onSelected = {},
			onBack = {},
		)
	}
}
