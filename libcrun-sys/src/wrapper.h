
#include <libcrun/error.h>
/* void libcrun_warning (const char *msg, ...); */

/* void libcrun_error (int errno_, const char *msg, ...); */

/* int libcrun_make_error (libcrun_error_t *err, int status, const char *msg, ...); */

/* void libcrun_error_write_warning_and_release (FILE *out, libcrun_error_t **err); */

/* void libcrun_fail_with_error (int errno_, const char *msg, ...) __attribute__ ((noreturn)); */

/* int libcrun_set_log_format (const char *format, libcrun_error_t *err); */

/* int libcrun_init_logging (crun_output_handler *output_handler, void **output_handler_arg, const char *id,
                                         const char *log, libcrun_error_t *err); */

/* int libcrun_error_release (libcrun_error_t *err); */

/* void libcrun_set_verbosity (int verbosity); */

/* int libcrun_get_verbosity (); */

#include <libcrun/utils.h>
/* int libcrun_str2sig (const char *name); */

#include <libcrun/container.h>
/* libcrun_container_t *libcrun_container_load_from_file (const char *path, libcrun_error_t *err); */

/* libcrun_container_t *libcrun_container_load_from_memory (const char *json, libcrun_error_t *err); */

/* void libcrun_container_free (libcrun_container_t *); */

/* int libcrun_container_run (libcrun_context_t *context, libcrun_container_t *container,
                                          unsigned int options, libcrun_error_t *error); */

/* int libcrun_container_delete (libcrun_context_t *context, runtime_spec_schema_config_schema *def,
                                             const char *id, bool force, libcrun_error_t *err); */

/* int libcrun_container_kill (libcrun_context_t *context, const char *id, int signal,
                                           libcrun_error_t *err); */

/* int libcrun_container_kill_all (libcrun_context_t *context, const char *id, int signal,
                                               libcrun_error_t *err); */

/* int libcrun_container_create (libcrun_context_t *context, libcrun_container_t *container,
                                             unsigned int options, libcrun_error_t *err); */

/* int libcrun_container_start (libcrun_context_t *context, const char *id, libcrun_error_t *err); */

/* int libcrun_container_state (libcrun_context_t *context, const char *id, FILE *out,
                                            libcrun_error_t *err); */

/* int libcrun_get_container_state_string (const char *id, libcrun_container_status_t *status,
                                                       const char *state_root, const char **container_status,
                                                       int *running, libcrun_error_t *err); */

/* int libcrun_container_exec (libcrun_context_t *context, const char *id,
                                           runtime_spec_schema_config_schema_process *process, libcrun_error_t *err); */

/* int libcrun_container_exec_process_file (libcrun_context_t *context, const char *id, const char *path,
                                                        libcrun_error_t *err); */

/* int libcrun_container_update (libcrun_context_t *context, const char *id, const char *content,
                                             size_t len, libcrun_error_t *err); */

/* int libcrun_container_update_from_file (libcrun_context_t *context, const char *id, const char *file,
                                                       libcrun_error_t *err); */

/* int libcrun_container_spec (bool root, FILE *out, libcrun_error_t *err); */

/* int libcrun_container_pause (libcrun_context_t *context, const char *id, libcrun_error_t *err); */

/* int libcrun_container_unpause (libcrun_context_t *context, const char *id, libcrun_error_t *err); */

/* int libcrun_container_checkpoint (libcrun_context_t *context, const char *id,
                                                 libcrun_checkpoint_restore_t *cr_options, libcrun_error_t *err); */

/* int libcrun_container_restore (libcrun_context_t *context, const char *id,
                                              libcrun_checkpoint_restore_t *cr_options, libcrun_error_t *err); */

#include <libcrun/seccomp_notify.h>
/* int libcrun_load_seccomp_notify_plugins (struct seccomp_notify_context_s **out, const char *plugins,
                                                        struct libcrun_load_seccomp_notify_conf_s *conf,
                                                        libcrun_error_t *err); */

/* int libcrun_seccomp_notify_plugins (struct seccomp_notify_context_s *ctx, int seccomp_fd,
                                                   libcrun_error_t *err); */

/* int libcrun_free_seccomp_notify_plugins (struct seccomp_notify_context_s *ctx, libcrun_error_t *err); */

#include <libcrun/cgroup.h>
/* int libcrun_get_cgroup_mode (libcrun_error_t *err); */

/* int libcrun_cgroup_killall_signal (const char *path, int signal, libcrun_error_t *err); */

/* int libcrun_cgroup_killall (const char *path, libcrun_error_t *err); */

/* int libcrun_cgroup_destroy (const char *id, const char *path, const char *scope, int manager,
                                           libcrun_error_t *err); */

/* int libcrun_move_process_to_cgroup (pid_t pid, pid_t init_pid, char *path, libcrun_error_t *err); */

/* int libcrun_update_cgroup_resources (int cgroup_mode,
                                                    runtime_spec_schema_config_linux_resources *resources, char *path,
                                                    libcrun_error_t *err); */

/* int libcrun_cgroup_is_container_paused (const char *cgroup_path, int cgroup_mode, bool *paused,
                                                       libcrun_error_t *err); */

/* int libcrun_cgroup_pause_unpause (const char *path, const bool pause, libcrun_error_t *err); */

/* int libcrun_cgroup_read_pids (const char *path, bool recurse, pid_t **pids, libcrun_error_t *err); */

#include <libcrun/status.h>
/* void libcrun_free_container_status (libcrun_container_status_t *status); */

/* int libcrun_write_container_status (const char *state_root, const char *id,
                                                   libcrun_container_status_t *status, libcrun_error_t *err); */

/* int libcrun_read_container_status (libcrun_container_status_t *status, const char *state_root,
                                                  const char *id, libcrun_error_t *err); */

/* void libcrun_free_containers_list (libcrun_container_list_t *list); */

/* int libcrun_is_container_running (libcrun_container_status_t *status, libcrun_error_t *err); */

/* char *libcrun_get_state_directory (const char *state_root, const char *id); */

/* int libcrun_container_delete_status (const char *state_root, const char *id, libcrun_error_t *err); */

/* int libcrun_get_containers_list (libcrun_container_list_t **ret, const char *state_root,
                                                libcrun_error_t *err); */
