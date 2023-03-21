package io.parity.signer.screens.keysetdetails.items

import SignerCheckbox
import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.parity.signer.R
import io.parity.signer.components.IdentIconWithNetwork
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.sharedcomponents.KeyPath
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.abbreviateString
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary

@Composable
fun KeyDerivedItemMultiselect(
	model: KeyModel,
	networkLogo: String,
	isSelected: Boolean = false,
	onClick: (Boolean, String) -> Unit,
) {
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = Color.Transparent,
		modifier = Modifier
			.padding(vertical = 16.dp)
			.clickable { onClick(!isSelected, model.addressKey) }
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
			IdentIconWithNetwork(
				identicon = model.identicon,
				networkLogoName = networkLogo,
				size = 36.dp,
				modifier = Modifier.padding(
					start = 16.dp,
					end = 12.dp
				)
			)
			Column(Modifier.weight(1f)) {
				if (model.path.isNotEmpty() || model.hasPwd) {
					KeyPath(
						path = model.path,
						hasPassword = model.hasPwd,
						textStyle = SignerTypeface.CaptionM,
						iconSize = 16.sp,
						textColor = MaterialTheme.colors.textTertiary,
						iconColor = MaterialTheme.colors.textTertiary,
					)
					Spacer(modifier = Modifier.padding(top = 4.dp))
				}
				Text(
					text = model.base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.BodyL,
				)
			}
			SignerCheckbox(
				isChecked = isSelected,
				modifier = Modifier
					.padding(horizontal = 8.dp)
			) {
				onClick(!isSelected, model.addressKey)
			}
		}
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
private fun PreviewKeyDerivedItemMultiselect() {
	SignerNewTheme {
		Column {
			KeyDerivedItemMultiselect(
				model = KeyModel.createStub(),
				networkLogo = "kusama",
				onClick = { _, _ -> },
			)
			SignerDivider()
			KeyDerivedItemMultiselect(
				model = KeyModel.createStub().copy(
					path = "//kusama//some//very_long_path//somesomesome", hasPwd = true,
				),
				networkLogo = "kusama",
				onClick = { _, _ -> },
			)
		}
	}
}
