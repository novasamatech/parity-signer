package io.parity.signer.screens.keysetdetails.items

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ExpandLess
import androidx.compose.material.icons.filled.ExpandMore
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.NetworkModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textDisabled


@Composable
fun NetworkKeysElementExpandable(network: NetworkModel, keys: List<KeyModel>) {
	val collapsed = remember { mutableStateOf(true) }

	Column(
		modifier = Modifier.background(
			MaterialTheme.colors.fill6,
			RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
		)
	) {
		//network row
		Row(
			modifier = Modifier.clickable { collapsed.value = !collapsed.value },
			verticalAlignment = Alignment.CenterVertically
		) {

			NetworkIcon(networkLogoName = network.logo,
				modifier = Modifier
					.padding(
						top = 16.dp,
						bottom = 16.dp,
						start = 16.dp,
						end = 12.dp
					)
					.size(36.dp),
			)
			Text(
				text = network.title,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
			)
			Spacer(modifier = Modifier.weight(1f))
			Box(
				modifier = Modifier
					.padding(top = 20.dp, bottom = 20.dp, end = 16.dp, start = 12.dp)
					.background(MaterialTheme.colors.fill6, CircleShape),
				contentAlignment = Alignment.Center,
			) {
				Image(
					imageVector = if (collapsed.value) {
						Icons.Filled.ExpandMore
					} else {
						Icons.Filled.ExpandLess
					},
					modifier = Modifier.size(20.dp),
					contentDescription = null,
					colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
				)
			}
		}
		if (!collapsed.value) {
			keys.forEach { key ->
				SignerDivider()
				KeyDerivedItem(model = key) {
//					todo dmitry finish
				}
			}
		}
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
private fun PreviewNetworkKeysElementExpandable() {
	SignerNewTheme {
		NetworkKeysElementExpandable(
			NetworkModel.createStub(),
			listOf(KeyModel.createStub())
		)
	}
}
