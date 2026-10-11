#!/bin/bash
# first argument is git repo url, second is path to rclone config file

#SBATCH --nodes=1
#SBATCH --gpus=1
#SBATCH --cpus-per-task=12

WORKDIR=${HOME}/git_slurm_ui/slurm_${SLURM_JOB_ID}
GIT_URL=$1
RCLONE_CONF=$2

mkdir "${WORKDIR}"

git clone "${GIT_URL}" "${WORKDIR}"

cd "${WORKDIR}"

chmod +x job.sh

./job.sh

TIME=$(date "+%m-%d-%Y_%H:%M")

rclone copy "${WORKDIR}" "gdrive:slurm_jobs/${TIME}" --config ${RCLONE_CONF}

cd /tmp
rm -rf "${WORKDIR}"