import type { Uri } from "vscode";
import type { Logger } from "monaco-languageclient/tools";
import { useWorkerFactory } from "monaco-editor-wrapper/workerFactory";
import { RegisteredMemoryFile } from "@codingame/monaco-vscode-files-service-override";
import type { IStoredWorkspace } from "@codingame/monaco-vscode-configuration-service-override";

export const disableButton = (id: string, disabled: boolean) => {
	const button = document.getElementById(id) as HTMLButtonElement | null;
	if (button !== null) {
		button.disabled = disabled;
	}
};

export const configureMonacoWorkers = (logger?: Logger) => {
	useWorkerFactory({
		workerOverrides: {
			ignoreMapping: true,
			workerLoaders: {
				TextEditorWorker: () =>
					new Worker(
						new URL(
							"monaco-editor/esm/vs/editor/editor.worker",
							import.meta.url
						),
					),
				TextMateWorker: () =>
					new Worker(
						new URL(
              "monaco-editor/esm/vs/editor/editor.worker",
							import.meta.url,
						),
					),
			},
		},
		logger,
	});
};

export const createDefaultWorkspaceFile = (
	workspaceFile: Uri,
	workspacePath: string,
) => {
	return new RegisteredMemoryFile(
		workspaceFile,
		JSON.stringify(
			<IStoredWorkspace>{
				folders: [
					{
						path: workspacePath,
					},
				],
			},
			null,
			2,
		),
	);
};
