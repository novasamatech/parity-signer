package io.parity.signer.components.sharedcomponents

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.outlined.Circle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.models.BASE58_STYLE_ABBREVIATE
import io.parity.signer.models.KeyCardModel
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.*


@Composable
fun KeyCard(model: KeyCardModel) {
	Row(
		Modifier
			.fillMaxWidth()
			.padding(16.dp)
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
				style = SignerTypeface.LabelS,
			)

			Spacer(Modifier.padding(top = 10.dp))

			Box(modifier = Modifier.padding(end = 24.dp)) {
				ShowBase58Collapsible(model.base58)
			}
		}


		//right()
		Column(horizontalAlignment = Alignment.End) {
			Box(contentAlignment = Alignment.TopEnd) {
				IdentIcon(model.identIcon, 36.dp)
				model.multiselect?.let {
					if (it) {
						Icon(
							Icons.Default.CheckCircle,
							"Not multiselected",
							tint = MaterialTheme.colors.Action400
						)
					} else {
						Icon(
							Icons.Outlined.Circle,
							"Multiselected",
							tint = MaterialTheme.colors.Action400
						)
					}
				}
			}

			Spacer(Modifier.padding(top = 14.dp))

			val networkName = model.network
			NetworkLabel(networkName)
		}
	}
}

@Composable
private fun NetworkLabel(networkName: String) {
	Box(
		modifier = Modifier
			.background(
				MaterialTheme.colors.fill12,
				RoundedCornerShape(12.dp)
			)
			.padding(horizontal = 8.dp, vertical = 2.dp),
		contentAlignment = Alignment.Center,
	) {
		Text(
			networkName,
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
		)
	}
}

@Composable
fun KeySeedCard(seedTitle: String, base58: String) {
	Column(
        Modifier
            .fillMaxWidth()
            .padding(16.dp)
	) {
		Text(
			seedTitle,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.LabelS,
		)
		ShowBase58Collapsible(base58)
	}
}

@Composable
fun ShowBase58Collapsible(base58: String) {
	val expanded = remember { mutableStateOf(false) }
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier.clickable { expanded.value = !expanded.value }
			.animateContentSize()
	) {
		if (expanded.value) {
			Text(
				base58,
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyM
			)
		} else {
			Text(
				base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyM,
				maxLines = 1,
			)
			Spacer(modifier = Modifier.padding(horizontal = 4.dp))
			Icon(
				imageVector = Icons.Default.KeyboardArrowDown,
				modifier = Modifier.size(20.dp),
				contentDescription = stringResource(R.string.description_expand_button),
				tint = MaterialTheme.colors.textSecondary
			)
		}
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
private fun PreviewKeyCard() {
	SignerNewTheme {
		KeyCard(model = KeyCardModel.createStub())
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
private fun PreviewKeySeedCard() {
	SignerNewTheme {
		KeySeedCard(
			seedTitle = "Seed title",
			base58 = KeyCardModel.createStub().base58,
		)
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
private fun PreviewNetworkLabel() {
	SignerNewTheme {
		Box(Modifier.size(width = 100.dp, height = 500.dp)) {
			NetworkLabel("Polkadot")
		}
	}
}
