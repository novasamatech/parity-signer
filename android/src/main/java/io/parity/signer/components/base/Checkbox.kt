import androidx.compose.animation.animateColorAsState
import androidx.compose.animation.core.tween
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.state.ToggleableState
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textTertiary

@Composable
fun SignerCheckboxColors(
	checkedColor: Color = MaterialTheme.colors.pink300,
	uncheckedColor: Color = MaterialTheme.colors.textTertiary,
	disabledColor: Color = MaterialTheme.colors.onSurface.copy(alpha = ContentAlpha.disabled),
	disabledIndeterminateColor: Color = checkedColor.copy(alpha = ContentAlpha.disabled)
): CheckboxColors {
	return remember(
		checkedColor,
		uncheckedColor,
		disabledColor,
		disabledIndeterminateColor,
	) {
		DefaultCheckboxColors(
			checkedBorderColor = checkedColor,
			checkedBoxColor = checkedColor.copy(alpha = 0f),
			checkedCheckmarkColor = checkedColor,
			uncheckedCheckmarkColor = uncheckedColor.copy(alpha = 0f),
			uncheckedBoxColor = checkedColor.copy(alpha = 0f),
			disabledCheckedBoxColor = disabledColor,
			disabledUncheckedBoxColor = disabledColor.copy(alpha = 0f),
			disabledIndeterminateBoxColor = disabledIndeterminateColor,
			uncheckedBorderColor = uncheckedColor,
			disabledBorderColor = disabledColor,
			disabledIndeterminateBorderColor = disabledIndeterminateColor,
		)
	}
}


@Stable
private class DefaultCheckboxColors(
	private val checkedCheckmarkColor: Color,
	private val uncheckedCheckmarkColor: Color,
	private val checkedBoxColor: Color,
	private val uncheckedBoxColor: Color,
	private val disabledCheckedBoxColor: Color,
	private val disabledUncheckedBoxColor: Color,
	private val disabledIndeterminateBoxColor: Color,
	private val checkedBorderColor: Color,
	private val uncheckedBorderColor: Color,
	private val disabledBorderColor: Color,
	private val disabledIndeterminateBorderColor: Color
) : CheckboxColors {
	private val BoxInDuration = 50
	private val BoxOutDuration = 100

	@Composable
	override fun checkmarkColor(state: ToggleableState): State<Color> {
		val target = if (state == ToggleableState.Off) {
			uncheckedCheckmarkColor
		} else {
			checkedCheckmarkColor
		}

		val duration =
			if (state == ToggleableState.Off) BoxOutDuration else BoxInDuration
		return animateColorAsState(target, tween(durationMillis = duration))
	}

	@Composable
	override fun boxColor(
		enabled: Boolean,
		state: ToggleableState
	): State<Color> {
		val target = if (enabled) {
			when (state) {
				ToggleableState.On, ToggleableState.Indeterminate -> checkedBoxColor
				ToggleableState.Off -> uncheckedBoxColor
			}
		} else {
			when (state) {
				ToggleableState.On -> disabledCheckedBoxColor
				ToggleableState.Indeterminate -> disabledIndeterminateBoxColor
				ToggleableState.Off -> disabledUncheckedBoxColor
			}
		}

		// If not enabled 'snap' to the disabled state, as there should be no animations between
		// enabled / disabled.
		return if (enabled) {
			val duration =
				if (state == ToggleableState.Off) BoxOutDuration else BoxInDuration
			animateColorAsState(target, tween(durationMillis = duration))
		} else {
			rememberUpdatedState(target)
		}
	}

	@Composable
	override fun borderColor(
		enabled: Boolean,
		state: ToggleableState
	): State<Color> {
		val target = if (enabled) {
			when (state) {
				ToggleableState.On, ToggleableState.Indeterminate -> checkedBorderColor
				ToggleableState.Off -> uncheckedBorderColor
			}
		} else {
			when (state) {
				ToggleableState.Indeterminate -> disabledIndeterminateBorderColor
				ToggleableState.On, ToggleableState.Off -> disabledBorderColor
			}
		}

		// If not enabled 'snap' to the disabled state, as there should be no animations between
		// enabled / disabled.
		return if (enabled) {
			val duration =
				if (state == ToggleableState.Off) BoxOutDuration else BoxInDuration
			animateColorAsState(target, tween(durationMillis = duration))
		} else {
			rememberUpdatedState(target)
		}
	}
}
