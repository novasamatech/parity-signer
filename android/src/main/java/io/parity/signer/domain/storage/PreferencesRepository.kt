package io.parity.signer.domain.storage

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.core.stringSetPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.single

private val Context.dataStore: DataStore<Preferences> by preferencesDataStore(name = "app_preferences")

class PreferencesRepository(private val context: Context) {

	private val networksFilterKey = stringSetPreferencesKey("network_filter")
	private val lastSelectedKeySet = stringPreferencesKey("last_selected_seed_name")

	val networksFilter = context.dataStore.data
		.map { preferences ->
			// No type safety.
			preferences[networksFilterKey] ?: emptySet()
		}

	suspend fun setNetworksFilter(newFilters: Set<String>) {
		context.dataStore.edit { settings ->
			settings[networksFilterKey] = newFilters
		}
	}

	suspend fun setLastSelectedSeed(seedName: String?) {
		context.dataStore.edit { settings ->
			when (seedName) {
				is String -> settings[lastSelectedKeySet] = seedName
				null -> settings -= lastSelectedKeySet
			}
		}
	}

	suspend fun getLastSelectedSeed(): String? {
		return context.dataStore.data.first()[lastSelectedKeySet]
	}
}
