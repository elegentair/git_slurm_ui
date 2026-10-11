#!/bin/bash
# first argument is git repo url, second is path to rclone config dir

#SBATCH --nodes=1
#SBATCH --gpus=1
#SBATCH --cpus-per-task=1

WORKDIR=${HOME}/git_slurm_ui/slurmdir_${SLURM_JOB_ID}
GIT_URL=$1
RCLONE_CONF=$2

git clone "${GIT_URL}" "${WORKDIR}"

cd "${WORKDIR}"

chmod +x job.sh
./job.sh

TIME=$(date "+%m-%d-%Y_%H:%M")

echo "Job Success!"

rclone copy "${WORKDIR}" "gdrive:slurm_jobs/${TIME}" --config "${RCLONE_CONF}/rclone.conf"

cd /tmp
rm -rf "${WORKDIR}"
rm -rf "${RCLONE_CONF}"