package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.tooling.preview.PreviewParameter
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Action400
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.uniffi.MscNetworkInfo

@Composable
fun NetworkCard(
	network: NetworkCardModel,
	selected: Boolean = false
) {

	Surface(
		shape = MaterialTheme.shapes.large,
		color = MaterialTheme.colors.Bg200,
		modifier = Modifier.height(47.dp)
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier
				.height(36.dp)
				.padding(horizontal = 20.dp)
		) {
			NetworkLogoName(
				logo = network.networkLogo,
				name = network.networkTitle,
			)
			Spacer(Modifier.weight(1f))
			if (selected) {
				Icon(
					Icons.Default.Check,
					"this network is selected",
					tint = MaterialTheme.colors.Action400
				)
			}
		}
	}
}

class NetworkCardModel(
	val networkTitle: String,
	val networkLogo: String,
) {

	constructor(network: MscNetworkInfo) : this(
		network.networkTitle,
		network.networkLogo,
	)
}



internal class NetworkCardPreviewParameter :
	PreviewParameterProvider<NetworkCardModel> {
	override val values: Sequence<NetworkCardModel> = sequenceOf(
		NetworkCardModel("Network Title", "Network Logo", )
	)
}

@Preview
@Composable
private fun NetworkCardPreview(
	@PreviewParameter(NetworkCardPreviewParameter::class) network: NetworkCardModel
) {
	NetworkCard(network)
}


