package io.parity.signer.components.sharedcomponents

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.outlined.Circle
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.models.BASE58_STYLE_ABBREVIATE
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.*

/**
 * Key card plate used in enter password screen, without network name
 */
@Composable
fun KeyCardPassword(model: KeyCardModelBase) {
	Row(
		Modifier
			.fillMaxWidth()
			.background(
				MaterialTheme.colors.red500fill12,
				RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
			)
			.padding(16.dp),
		verticalAlignment = Alignment.CenterVertically,
	) {

		//left
		Column(Modifier.weight(1f)) {
			Row(
				verticalAlignment = Alignment.CenterVertically
			) {
				Text(
					model.path,
					color = MaterialTheme.colors.textSecondary,
					style = SignerTypeface.CaptionM,
				)
				if (model.hasPwd) {
					Text(
						" •••• ",
						color = MaterialTheme.colors.textSecondary,
						style = SignerTypeface.CaptionM,
					)
					Icon(
						Icons.Default.Lock,
						contentDescription = stringResource(R.string.description_locked_icon),
						tint = MaterialTheme.colors.textSecondary,
					)
				}
			}

			Spacer(Modifier.padding(top = 4.dp))

			Text(
				model.seedName,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyM,
			)

			Spacer(Modifier.padding(top = 4.dp))

			Box(modifier = Modifier.padding(end = 24.dp)) {
				Text(
					text = model.base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.BodyM,
					maxLines = 1,
				)
			}
		}

		//right()
		IdentIcon(model.identIcon, 36.dp)
	}
}


@Preview(
	name = "day",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark theme",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewKeyCardPassword() {
	SignerNewTheme {
		KeyCardPassword(
			KeyCardModelBase.createStub(),
		)
	}
}