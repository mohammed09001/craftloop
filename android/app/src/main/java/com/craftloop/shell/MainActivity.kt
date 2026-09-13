// Execution 01, Phase 28, Task 203/206. Placeholder shell -- see
// ../../../../../README.md. Deliberately one screen, one string, one
// real call into the shared Rust core through the UniFFI-generated
// bindings this repository already proved generate correctly
// (../../../../../../execution-evidence/mobile-ffi/generated-kotlin/).
// No navigation, no drawing surface, no design system -- Task 206 is
// explicit that Execution 01 does not spend its budget on final visual
// design here.

package com.craftloop.shell

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import uniffi.craftloop_mobile_ffi.FfiCommandNamespace
import uniffi.craftloop_mobile_ffi.resolveCommand

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // One real call across the FFI boundary Task 201 defined,
        // proving this shell is wired to the shared core rather than a
        // static mock -- nothing more.
        val result = resolveCommand("pen", FfiCommandNamespace.NOTEBOOK)

        setContent {
            MaterialTheme {
                Surface {
                    Text("Craft Loop shell placeholder. resolveCommand(\"pen\") = $result")
                }
            }
        }
    }
}
