package io.parity.signer.components.items

import SignerCheckbox
import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.models.BASE58_STYLE_ABBREVIATE
import io.parity.signer.models.KeyModel
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary

@Composable
fun KeyDerivedItemMultiselect(
	model: KeyModel,
	isSelected: Boolean = false,
	onClick: (Boolean, String) -> Unit,
) {
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = Color.Transparent,
		modifier = Modifier.clickable { onClick(!isSelected, model.addressKey) }
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
			IdentIcon(
				identicon = model.identicon, size = 36.dp, modifier = Modifier.padding(
					top = 16.dp,
					bottom = 16.dp,
					start = 16.dp,
					end = 12.dp
				)
			)
			Column(Modifier.weight(1f)) {
				Row(verticalAlignment = Alignment.CenterVertically) {
					Text(
						text = model.path,
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.LabelM,
					)
					if (model.hasPwd) {
						Icon(
							painterResource(id = R.drawable.ic_lock_16),
							contentDescription = stringResource(R.string.key_lock_item),
							tint = MaterialTheme.colors.textTertiary,
							modifier = Modifier.padding(start = 8.dp)
						)
					}
				}
				Spacer(modifier = Modifier.padding(top = 4.dp))
				Text(
					text = model.base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.BodyM,
				)
			}
			SignerCheckbox(
				isChecked = isSelected,
				modifier = Modifier
					.padding(end = 8.dp)
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
		KeyDerivedItemMultiselect(
			model = KeyModel.createStub(),
			onClick = { _, _ -> },
		)
	}
}
