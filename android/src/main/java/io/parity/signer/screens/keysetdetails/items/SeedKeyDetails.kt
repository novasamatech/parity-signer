package io.parity.signer.screens.keysetdetails.items

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.abbreviateString
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill12
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.ui.theme.textTertiary


@Composable
fun SeedKeyDetails(
	model: KeyModel,
	onShowRoot: Callback,
	onSeedSelect: Callback,
	modifier: Modifier = Modifier,
) {
	Column(
		modifier = modifier
			.fillMaxWidth(),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		IdentIconImage(
			identicon = model.identicon,
			modifier = Modifier.clickable(onClick = onShowRoot),
			size = 56.dp
		)
		//key name
		Row(
			modifier = Modifier.clickable(onClick = onSeedSelect),
			verticalAlignment = Alignment.CenterVertically,
		) {
			Text(
				text = model.seedName,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleXl,
				textAlign = TextAlign.Center
			)
			Spacer(modifier = Modifier.padding(horizontal = 4.dp))
			Icon(
				imageVector = Icons.Default.KeyboardArrowDown,
				modifier = Modifier.size(32.dp),
				contentDescription = stringResource(R.string.description_expand_button),
				tint = MaterialTheme.colors.textSecondary
			)
		}
		Text(
			model.base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyM,
			maxLines = 1,
			modifier = Modifier
				.padding(top = 8.dp)
				.clickable(onClick = onShowRoot)
				.background(
					MaterialTheme.colors.fill12,
					RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
				)
				.padding(horizontal = 16.dp, vertical = 4.dp)
		)
	}
}


@Preview(
	name = "light",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewKeySeedCard() {
	SignerNewTheme {
		SeedKeyDetails(KeyModel.createStub()
			.copy(identicon = PreviewData.Identicon.jdenticonIcon),
			{},
			{},
			)
	}
}
