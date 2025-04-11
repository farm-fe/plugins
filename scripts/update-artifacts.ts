import * as fs from 'fs';
import * as path from 'path';
import * as yaml from 'yaml';

const WORKFLOW_FILE = '.github/workflows/build.yaml';
const RUST_PLUGINS_DIR = 'rust-plugins';

function generateUploadStep(pluginName: string): object {
  return {
    name: `Upload Plugin ${pluginName}`,
    uses: 'actions/upload-artifact@v4',
    with: {
      name: `\${{ github.sha }}-\${{ matrix.settings.abi }}-${pluginName}`,
      path: `./rust-plugins/${pluginName}/npm/\${{ matrix.settings.abi }}/index.farm`,
      'if-no-files-found': 'ignore'
    }
  };
}

function updateWorkflow() {
  // Read the workflow file
  const workflowPath = path.join(process.cwd(), WORKFLOW_FILE);
  const workflowContent = fs.readFileSync(workflowPath, 'utf8');
  const workflow = yaml.parse(workflowContent);

  // Get all rust plugin directories
  const pluginsPath = path.join(process.cwd(), RUST_PLUGINS_DIR);
  const plugins = fs.readdirSync(pluginsPath)
    .filter(name => fs.statSync(path.join(pluginsPath, name)).isDirectory());

  // Find the build job
  const buildJob = workflow.jobs.build;
  if (!buildJob) {
    throw new Error('Could not find build job in workflow file');
  }

  // Remove existing upload steps
  // @ts-ignore
  buildJob.steps = buildJob.steps.filter(step => 
    !(step.name && step.name.startsWith('Upload Plugin '))
  );

  // Add new upload steps for each plugin
  plugins.forEach(plugin => {
    buildJob.steps.push(generateUploadStep(plugin));
  });

  // Write back to file
  fs.writeFileSync(workflowPath, yaml.stringify(workflow));
  
  console.log(`Updated workflow file with ${plugins.length} plugins`);
}

updateWorkflow();