pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
gnome = import('gnome')

gnome.compile_resources('toolbx-tuner',
  'toolbx_tuner.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'toolbx-tuner')
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
conf.set_quoted('PKGDATADIR', pkgdatadir)

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src', 'config.rs'),
  join_paths(meson.project_source_root(), 'src', 'config.rs'),
  check: true
)

rust_sources = files(
  'main.rs',
  'toolbx/mod.rs',
  'ui/mod.rs',
  'ui/ui_strings.rs',
  'ui/app/mod.rs',
  'ui/app/model.rs',
  'ui/app/widgets.rs',
  'ui/app/workers.rs',
  'ui/app/messages.rs',
  'ui/app/toolbox_list.rs',
  'ui/app/update.rs',
  'ui/components/mod.rs',
  'ui/components/toolbox_apps/factory.rs',
  'ui/components/toolbox_apps/messages.rs',
  'ui/components/toolbox_apps/mod.rs',
  'ui/components/toolbox_apps/model.rs',
  'ui/components/toolbox_apps/update.rs',
  'ui/components/toolbox_apps/widgets.rs',
  'ui/components/toolbox_settings/messages.rs',
  'ui/components/toolbox_settings/mod.rs',
  'ui/components/toolbox_settings/model.rs',
  'ui/components/toolbox_settings/update.rs',
  'ui/components/toolbox_settings/widgets.rs',  
)

sources = [cargo_sources, rust_sources]

cargo_script = find_program(join_paths(meson.project_source_root(), 'build-aux/cargo.sh'))
cargo_release = custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo_script,
    meson.project_build_root(),
    meson.project_source_root(),
    '@OUTPUT@',
    get_option('buildtype'),
    meson.project_name(),
  ]
)
