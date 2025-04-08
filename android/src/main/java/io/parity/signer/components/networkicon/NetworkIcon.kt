package io.parity.signer.components.networkicon

import android.annotation.SuppressLint
import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.AutoSizeText
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun NetworkIcon(
	networkLogoName: String,
	modifier: Modifier = Modifier,
	size: Dp = 32.dp,
) {
	val icon = getIconForNetwork(networkLogoName.lowercase())
	if (icon != null) {
		Image(
			painter = icon,
			contentDescription = null,
			modifier = modifier
				.clip(CircleShape)
				.size(size),
		)
	} else {
		val networkColors = ServiceLocator.unknownNetworkColorsGenerator
			.getBackground(networkLogoName.lowercase())
			.toUnknownNetworkColorsDrawable()
		val chars = networkLogoName.take(1).uppercase()
		UnknownNetworkIcon(networkColors, chars, size, modifier)
	}
}

@Composable
private fun UnknownNetworkIcon(
	networkColors: UnknownNetworkColorDrawable,
	chars: String,
	size: Dp,
	modifier: Modifier = Modifier
) {
	Box(
		modifier = modifier
			.size(size)
			.background(networkColors.background, CircleShape),
		contentAlignment = Alignment.Center
	) {
		AutoSizeText(
			text = chars,
			fontWeight = FontWeight.Bold,
			color = networkColors.text,
		)
	}
}

@Composable
@SuppressLint("DiscouragedApi")
private fun getIconForNetwork(networkName: String): Painter? {
//	val resource = resources.getIdentifier(/* name = */ "network_$networkName",
//		/* defType = */"drawable",/* defPackage = */packageName)

	val id = getResourceIdForNetwork(networkName)

	return if (id > 0) {
		painterResource(id = id)
	} else {
		null
	}
}

/**
 * Those icons and names taken from iOS where they taken from
 * https://metadata.novasama.io/
 * It is used just to show some nice icons for known networks, orherwise
 * generated unknown icon will be shown
 */
@Composable
private fun getResourceIdForNetwork(networkName: String) =
	when (networkName) {
		//those svg's are not supported by android vector drawable -
		// too big or links to text png inside. Added as png
		"composable" -> R.drawable.network_composable
		"mangata-parachain" -> R.drawable.network_mangata_parachain
		//svgs below
		"acala" -> R.drawable.network_acala
		"acala_testnet" -> R.drawable.network_acala_testnet
		"ajuna" -> R.drawable.network_ajuna
		"aleph-node" -> R.drawable.network_aleph_node
		"alphanet" -> R.drawable.network_alphanet
		"altair" -> R.drawable.network_altair
		"amplitude" -> R.drawable.network_amplitude
		"analog-testnet" -> R.drawable.network_analog
		"analog-timechain" -> R.drawable.network_analog
		"appchain" -> R.drawable.network_appchain
		"apron" -> R.drawable.network_apron
		"ares" -> R.drawable.network_ares
		"astar" -> R.drawable.network_astar
		"aventus" -> R.drawable.network_aventus
		"bajun" -> R.drawable.network_bajun
		"basilisk" -> R.drawable.network_basilisk
		"beresheet" -> R.drawable.network_beresheet
		"bifrost" -> R.drawable.network_bifrost
		"bifrost_polkadot" -> R.drawable.network_bifrost_polkadot
		"bitgreen" -> R.drawable.network_bitgreen
		"calamari" -> R.drawable.network_calamari
		"centrifuge" -> R.drawable.network_centrifuge
		"chainx" -> R.drawable.network_chainx
		"clover-mainnet" -> R.drawable.network_clover_mainnet
		"clover-solochain" -> R.drawable.network_clover_solochain
		"coinversation" -> R.drawable.network_coinversation
		"continuum-runtime" -> R.drawable.network_continuum_runtime
		"Crab Parachain" -> R.drawable.network_crab_parachain
		"crust-collator" -> R.drawable.network_crust_collator
		"Crust" -> R.drawable.network_crust
		"Darwinia Parachain" -> R.drawable.network_darwinia_parachain
		"datahighway-parachain" -> R.drawable.network_datahighway_parachain
		"DICO" -> R.drawable.network_dico
		"DORA KSM Parachain" -> R.drawable.network_dora_ksm_parachain
		"edgeware" -> R.drawable.network_edgeware
		"efinity" -> R.drawable.network_efinity
		"encointer-parachain" -> R.drawable.network_encointer_parachain
		"equilibrium" -> R.drawable.network_equilibrium
		"frequency" -> R.drawable.network_frequency
		"frequency-rococo" -> R.drawable.network_frequency
		"fusotao" -> R.drawable.network_fusotao
		"geminis" -> R.drawable.network_geminis
		"genshiro" -> R.drawable.network_genshiro
		"governance2_testnet" -> R.drawable.network_governance2_testnet
		"hashed" -> R.drawable.network_hashed
		"heiko" -> R.drawable.network_heiko
		"hydradx" -> R.drawable.network_hydradx
		"imbue" -> R.drawable.network_imbue
		"Integritee_Kusama" -> R.drawable.network_integritee_kusama
		"integritee-parachain" -> R.drawable.network_integritee_parachain
		"Integritee_Polkadot" -> R.drawable.network_integritee_polkadot
		"interlay-parachain" -> R.drawable.network_interlay_parachain
		"ipci" -> R.drawable.network_ipci
		"kabocha-parachain" -> R.drawable.network_kabocha_parachain
		"karura" -> R.drawable.network_karura
		"khala" -> R.drawable.network_khala
		"kico" -> R.drawable.network_kico
		"kilt-peregrine" -> R.drawable.network_kilt_peregrine
		"kilt-spiritnet" -> R.drawable.network_kilt_spiritnet
		"kintsugi-parachain" -> R.drawable.network_kintsugi_parachain
		"kpron" -> R.drawable.network_kpron
		"kulupu" -> R.drawable.network_kulupu
		"kusama" -> R.drawable.network_kusama
		"kylin" -> R.drawable.network_kylin
		"listen" -> R.drawable.network_listen
		"litentry-parachain" -> R.drawable.network_litentry_parachain
		"litmus-parachain" -> R.drawable.network_litmus_parachain
		"loom" -> R.drawable.network_loom
		"mainnet" -> R.drawable.network_mainnet
		"mangata" -> R.drawable.network_mangata
		"manta" -> R.drawable.network_manta
		"mars" -> R.drawable.network_mars
		"moonbase" -> R.drawable.network_moonbase
		"moonbeam" -> R.drawable.network_moonbeam
		"moonriver" -> R.drawable.network_moonriver
		"node" -> R.drawable.network_node
		"nodle-para" -> R.drawable.network_nodle_para
		"oak" -> R.drawable.network_oak
		"omnibtc" -> R.drawable.network_omnibtc
		"origintrail-parachain" -> R.drawable.network_origintrail_parachain
		"parallel" -> R.drawable.network_parallel
		"Pendulum" -> R.drawable.network_pendulum
		"phala" -> R.drawable.network_phala
		"picasso" -> R.drawable.network_picasso
		"pichiu" -> R.drawable.network_pichiu
		"pioneer-runtime" -> R.drawable.network_pioneer_runtime
		"polkadex-parachain" -> R.drawable.network_polkadex_parachain
		"polkadot" -> R.drawable.network_polkadot
		"polkafoundry" -> R.drawable.network_polkafoundry
		"polkasmith" -> R.drawable.network_polkasmith
		"polymesh_mainnet" -> R.drawable.network_polymesh_mainnet
		"polymesh_testnet" -> R.drawable.network_polymesh_testnet
		"pontemnox" -> R.drawable.network_pontemnox
		"pontem" -> R.drawable.network_pontem
		"quartz" -> R.drawable.network_quartz
		"robonomics" -> R.drawable.network_robonomics
		"rococo" -> R.drawable.network_rococo
		"sakura" -> R.drawable.network_sakura
		"sherpax" -> R.drawable.network_sherpax
		"shiden" -> R.drawable.network_shiden
		"singular" -> R.drawable.network_singular
		"snow" -> R.drawable.network_snow
		"sora-parachain" -> R.drawable.network_sora_parachain
		"sora" -> R.drawable.network_sora
		"standard" -> R.drawable.network_standard
		"statemine" -> R.drawable.network_statemine
		"statemint" -> R.drawable.network_statemint
		"subdao" -> R.drawable.network_subdao
		"subgame-gamma" -> R.drawable.network_subgame_gamma
		"Subgame" -> R.drawable.network_subgame
		"subsocial-parachain" -> R.drawable.network_subsocial_parachain
		"subsocial-solochain" -> R.drawable.network_subsocial_solochain
		"template-parachain" -> R.drawable.network_template_parachain
		"tinkernet_node" -> R.drawable.network_tinkernet_node
		"totem-parachain" -> R.drawable.network_totem_parachain
		"turing-staging" -> R.drawable.network_turing_staging
		"turing" -> R.drawable.network_turing
		"unique" -> R.drawable.network_unique
		"westend" -> R.drawable.network_westend
		"westmint" -> R.drawable.network_westmint
		"xxnetwork" -> R.drawable.network_xxnetwork
		"zeitgeist" -> R.drawable.network_zeitgeist
		else -> -1
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
private fun PreviewEmptyIcon() {
	SignerNewTheme {
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
		) {
			NetworkIcon("")
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
private fun PreviewNetworkIconSizes() {
	SignerNewTheme {
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
		) {
			NetworkIcon("polkadot")
			NetworkIcon("some_unknown")
			NetworkIcon("polkadot", size = 18.dp)
			NetworkIcon("some_unknown2", size = 18.dp)
			NetworkIcon("polkadot", size = 56.dp)
			NetworkIcon("some_unknown3", size = 56.dp)
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
private fun PreviewNetworkIconUnknownIcons() {
	SignerNewTheme {
		Column {
			val colors = UnknownNetworkColors.values()
			colors.forEach { color ->
				UnknownNetworkIcon(
					networkColors = color.toUnknownNetworkColorsDrawable(),
					chars = "W",
					size = 24.dp
				)
			}
		}
	}
}



