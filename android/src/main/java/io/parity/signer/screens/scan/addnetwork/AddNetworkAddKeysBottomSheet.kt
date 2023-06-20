package io.parity.signer.screens.scan.addnetwork

import SignerCheckbox
import android.annotation.SuppressLint
import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.RowButtonsBottomSheet
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6


@Composable
fun AddNetworkAddKeysBottomSheet(
	networkTitle: String,
	seeds: List<String>,
	onCancel: Callback,
	onDone: (seeds: List<String>) -> Unit,
) {
	var selected = remember { mutableSetOf<String>() }
	AddNetworkAddKeysBottomSheet(
		networkTitle = networkTitle,
		seeds = seeds,
		selectedSeeds = selected,
		onAddKeyset = { keyset ->
			if (selected.contains(keyset)) {
				selected.remove(keyset)
			} else {
				selected.add(keyset)
			}
		},
		onAddAll = {
			if (selected.size >= seeds.size) {
				selected = mutableSetOf()
			} else {
				selected = seeds.toMutableSet()
			}
		},
		onCancel = onCancel,
		onDone = { onDone(selected.toList()) },
	)
}

@Composable
private fun AddNetworkAddKeysBottomSheet(
	networkTitle: String,
	seeds: List<String>,
	selectedSeeds: Set<String>,
	onAddKeyset: (String) -> Unit,
	onAddAll: Callback,
	onCancel: Callback,
	onDone: Callback,
) {
	Column(
		modifier = Modifier
			.background(MaterialTheme.colors.background)
	) {
		Column(
			modifier = Modifier
                .weight(1f, fill = false)
                .verticalScroll(rememberScrollState()),
		) {
			Text(
				text = stringResource(
					R.string.add_network_add_keys_list_title,
					networkTitle
				),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
				modifier = Modifier
                    .padding(horizontal = 24.dp)
                    .padding(top = 32.dp, bottom = 24.dp),
			)
			Text(
				text = stringResource(R.string.add_network_add_keys_list_subtitle),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier
					.padding(horizontal = 24.dp)
			)
			Column(
				modifier = Modifier
                    .padding(8.dp)
                    .background(
                        MaterialTheme.colors.fill6,
                        RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
                    )
			) {
				seeds.forEach { keyset ->
					KeysetItemMultiselect(
						keyset = keyset,
						isSelected = selectedSeeds.contains(keyset),
						onClick = { network -> onAddKeyset(network) },
					)
					SignerDivider()
				}
				KeysetItemMultiselectAll(onAddAll)
			}
		}
		RowButtonsBottomSheet(
			labelCancel = stringResource(R.string.generic_cancel),
			labelCta = stringResource(R.string.add_network_add_keys_cta),
			onClickedCancel = onCancel,
			onClickedCta = onDone,
			modifier = Modifier.padding(horizontal = 32.dp, vertical = 24.dp),
			isCtaEnabled = selectedSeeds.isNotEmpty()
		)
	}
}


@Composable
private fun KeysetItemMultiselect(
	keyset: String,
	isSelected: Boolean,
	onClick: (String) -> Unit,
) {
	Row(
		modifier = Modifier.clickable { onClick(keyset) },
		verticalAlignment = Alignment.CenterVertically
	) {
		Text(
			text = keyset,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
			modifier = Modifier
                .weight(1f)
                .padding(
                    top = 16.dp,
                    bottom = 16.dp,
                    start = 16.dp,
                    end = 12.dp,
                )
		)
		SignerCheckbox(
			isChecked = isSelected,
			modifier = Modifier.padding(end = 8.dp),
			uncheckedColor = MaterialTheme.colors.primary,
		) {
			onClick(keyset)
		}
	}
}


@Composable
private fun KeysetItemMultiselectAll(
	onClick: Callback,
) {
	Row(
		modifier = Modifier
            .clickable(onClick = onClick)
            .height(68.dp)
            .padding(horizontal = 16.dp)
            .fillMaxWidth(1f),
		verticalAlignment = Alignment.CenterVertically
	) {
		Text(
			text = stringResource(R.string.keyset_create_keys_select_all),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
		)
	}
}


@SuppressLint("UnrememberedMutableState")
@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewAddNetworkAddKeysBottomSheet() {
	SignerNewTheme {
		AddNetworkAddKeysBottomSheet(
			networkTitle = "Ascend",
			seeds = listOf(
				"My special key",
				"Special",
				"Main",
				"Very very very very very vey vcey vey very vey long keyset"
			),
			selectedSeeds = setOf("Special"),
			onAddKeyset = {},
			onAddAll = {},
			onCancel = {},
			onDone = {},
		)
	}
}


@SuppressLint("UnrememberedMutableState")
@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewAddNetworkAddKeysBottomSheetVeryLong() {
	SignerNewTheme {
		AddNetworkAddKeysBottomSheet(
			networkTitle = "Ascend",
			seeds = listOf(
				"My special key",
				"Special",
				"Main",
				"Very very very very very vey vcey vey very vey long keyset",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
				"some more",
			),
			selectedSeeds = setOf(),
			onAddKeyset = {},
			onAddAll = {},
			onCancel = {},
			onDone = {},
		)
	}
}
