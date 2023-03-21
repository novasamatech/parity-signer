package io.parity.signer.screens.keysetdetails.items

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
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
import androidx.compose.runtime.MutableState
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
fun NetworkKeysExpandableMultiselect(
	network: NetworkModel,
	keys: List<KeyModel>,
	selectedKeysAdr: Set<String>,
	onKeyClick: (isSelected: Boolean, keyAddress: String) -> Unit,
) {
	val collapsed = remember { mutableStateOf(false) }
	NetworkKeysExpandableMultiselectPrivate(
		network,
		keys,
		selectedKeysAdr,
		collapsed,
		onKeyClick
	)
}

@Composable
private fun NetworkKeysExpandableMultiselectPrivate(
	network: NetworkModel,
	keys: List<KeyModel>,
	selectedKeysAdr: Set<String>,
	collapsed: MutableState<Boolean>,
	onKeyClick: (isSelected: Boolean, keyAddress: String) -> Unit,
) {
	Column(
		modifier = Modifier
			.background(
				MaterialTheme.colors.fill6,
				RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
			)
			.animateContentSize()
	) {
		//network row
		Row(
			modifier = Modifier.clickable { collapsed.value = !collapsed.value },
			verticalAlignment = Alignment.CenterVertically
		) {

			NetworkIcon(
				networkLogoName = network.logo,
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
					.padding(
						top = 20.dp,
						bottom = 20.dp,
						end = 16.dp,
						start = 12.dp
					)
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
			var first = true
			keys.forEach { key ->
				SignerDivider(modifier = if (first) Modifier else Modifier.padding(start = 48.dp))
				KeyDerivedItemMultiselect(
					model = key,
					networkLogo = network.logo,
					isSelected = selectedKeysAdr.contains(key.addressKey),
					onClick = { isSelected, address -> onKeyClick(isSelected, address) })
				first = false
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
		NetworkKeysExpandable(
			NetworkModel.createStub(),
			listOf(KeyModel.createStub())
		) { _, _ -> }
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
private fun PreviewNetworkKeysExpandableMultiselect() {
	SignerNewTheme {
		val key = KeyModel.createStub()
		NetworkKeysExpandableMultiselect(
			NetworkModel.createStub(),
			listOf(
				key,
				key.copy(
					path = key.path + "//somemore",
					addressKey = "f1c25182fb8e20313b2c1eb49219da7a70c"
				)
			),
			setOf(key.addressKey),
		) { _, _ -> }
	}
}


