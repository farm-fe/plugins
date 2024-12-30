import * as vscode from "vscode";
import {
	RegisteredFileSystemProvider,
	registerFileSystemOverlay,
	RegisteredMemoryFile,
} from "@codingame/monaco-vscode-files-service-override";
import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { MonacoEditorReactComp } from "@typefox/monaco-editor-react";
import type {
	MonacoEditorLanguageClientWrapper,
	TextChanges,
} from "monaco-editor-wrapper";
import { createUserConfig } from "./config.js";
import badPyCode from "./bad.py?raw";

export const runPythonReact = async () => {
	const badPyUri = vscode.Uri.file("/workspace/bad.py");
	const fileSystemProvider = new RegisteredFileSystemProvider(false);
	fileSystemProvider.registerFile(
		new RegisteredMemoryFile(badPyUri, badPyCode),
	);
	registerFileSystemOverlay(1, fileSystemProvider);

	const onTextChanged = (textChanges: TextChanges) => {
		console.log(
			`Dirty? ${textChanges.isDirty}\ntext: ${textChanges.modified}\ntextOriginal: ${textChanges.original}`,
		);
	};
	const wrapperConfig = createUserConfig(
		"/workspace",
		badPyCode,
		"/workspace/bad.py",
	);
	// biome-ignore lint/style/noNonNullAssertion: <explanation>
	const root = ReactDOM.createRoot(document.getElementById("react-root")!);

	try {
		document
			.querySelector("#button-start")
			?.addEventListener("click", async () => {
				const App = () => {
					return (
						<div style={{ height: "80vh", padding: "5px" }}>
							<MonacoEditorReactComp
								wrapperConfig={wrapperConfig}
								style={{ height: "100%" }}
								onTextChanged={onTextChanged}
								onLoad={(wrapper: MonacoEditorLanguageClientWrapper) => {
									console.log(
										`Loaded ${wrapper.reportStatus().join("\n").toString()}`,
									);
								}}
								onError={(e) => {
									console.error(e);
								}}
							/>
						</div>
					);
				};

				const strictMode =
					// biome-ignore lint/style/noNonNullAssertion: <explanation>
					(document.getElementById("checkbox-strict-mode")! as HTMLInputElement)
						.checked;

				if (strictMode) {
					root.render(
						<StrictMode>
							<App />
						</StrictMode>,
					);
				} else {
					root.render(<App />);
				}
			});
		document.querySelector("#button-dispose")?.addEventListener("click", () => {
			root.render([]);
		});
	} catch (e) {
		console.error(e);
	}
};
