# -*- mode: ruby -*-
# vi: set ft=ruby :

# Vagrantfile API/syntax version. Don't touch unless you know what you're doing!
VAGRANTFILE_API_VERSION = "2"

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|
  config.vm.box = "base"
  config.vm.box_url = 'http://files.vagrantup.com/precise64_vmware.box'

  config.vm.provider "vmware_fusion" do |v|
    v.gui = false
    v.vmx["memsize"] = "1024"
    v.vmx["numvcpus"] = "2"
  end

  ["Gemfile", "kannel.conf", "kannel_test.rb", "setup.sh", "modems.conf", "start_example.sh"].each do |file_name|
    config.vm.provision "file", source: "./#{file_name}", destination: "~/#{file_name}"
  end
end
