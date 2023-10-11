## Install

`brew install ansible`

## Create server:

`ansible-playbook -i hosts.ini setup.yml`

## Create new user (replace xogames to your user):

`ansible-playbook -i hosts.ini new_user.yml -e "user=xogames"`
