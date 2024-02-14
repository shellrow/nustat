<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
//import { listen } from '@tauri-apps/api/event';
import { setRoutine } from '../libnp/routine';
import { Overview } from '../types/np-types';

const autoUpdate = ref(true);
const overview = ref<Overview>();

const routine = setRoutine({
  interval: 5000,
  callback: () => { 
        if (autoUpdate.value) {
            GetOverview(); 
        }
    }
});

const GetOverview = async() => {
    const result = await invoke<Overview>('get_overview');
    overview.value = result;
}

onMounted(() => {
    GetOverview();
    routine.start();
});

onUnmounted(() => {
    routine.stop();
});

</script>
<template>
    <div class="flex flex-column flex-auto">
        
        <div class="p-5">
            <div class="grid">
                <div class="col-12 lg:col-6 xl:col-3">
                    <div class="surface-card shadow-2 p-3 border-1 border-50 border-round">
                        <div class="flex justify-content-between mb-3">
                            <div>
                                <span class="block text-500 font-medium mb-3">Packets</span>
                                <div class="text-900 font-medium text-xl">{{ overview?.captured_packets || 0 }}</div>
                            </div>
                            <div class="flex align-items-center justify-content-center bg-blue-100 border-round" style="width:2.5rem;height:2.5rem">
                                <i class="pi pi-arrow-right-arrow-left text-blue-500 text-xl"></i>
                            </div>
                        </div>
                        <span class="text-green-500 font-medium"> {{ (overview?.traffic.bytes_received || 0) + (overview?.traffic.bytes_sent || 0) }} </span>
                        <span class="text-500"> bytes captured</span>
                    </div>
                </div>
                <div class="col-12 lg:col-6 xl:col-3">
                    <div class="surface-card shadow-2 p-3 border-1 border-50 border-round">
                        <div class="flex justify-content-between mb-3">
                            <div>
                                <span class="block text-500 font-medium mb-3">Network Interface</span>
                                <div class="text-900 font-medium text-xl">{{ overview?.if_name }}</div>
                            </div>
                            <div class="flex align-items-center justify-content-center bg-orange-100 border-round" style="width:2.5rem;height:2.5rem">
                                <i class="pi pi-cog text-orange-500 text-xl"></i>
                            </div>
                        </div>
                        <span class="text-500">Interface Index: </span>
                        <span class="text-green-500 font-medium">{{ overview?.if_index }}</span>
                    </div>
                </div>
                <div class="col-12 lg:col-6 xl:col-3">
                    <div class="surface-card shadow-2 p-3 border-1 border-50 border-round">
                        <div class="flex justify-content-between mb-3">
                            <div>
                                <span class="block text-500 font-medium mb-3">Ingress</span>
                                <div class="text-900 font-medium text-xl">{{ overview?.traffic.bytes_received }} bytes</div>
                            </div>
                            <div class="flex align-items-center justify-content-center bg-cyan-100 border-round" style="width:2.5rem;height:2.5rem">
                                <i class="pi pi-download text-cyan-500 text-xl"></i>
                            </div>
                        </div>
                        <span class="text-green-500 font-medium">{{ overview?.traffic.packet_received }}</span>
                        <span class="text-500"> packets received</span>
                    </div>
                </div>
                <div class="col-12 lg:col-6 xl:col-3">
                    <div class="surface-card shadow-2 p-3 border-1 border-50 border-round">
                        <div class="flex justify-content-between mb-3">
                            <div>
                                <span class="block text-500 font-medium mb-3">Egress</span>
                                <div class="text-900 font-medium text-xl">{{ overview?.traffic.bytes_sent }} bytes</div>
                            </div>
                            <div class="flex align-items-center justify-content-center bg-purple-100 border-round" style="width:2.5rem;height:2.5rem">
                                <i class="pi pi-upload text-purple-500 text-xl"></i>
                            </div>
                        </div>
                        <span class="text-green-500 font-medium">{{ overview?.traffic.packet_sent }}</span>
                        <span class="text-500"> packets sent</span>
                    </div>
                </div>
                <div class="col-12">
                    <div class="surface-card shadow-2 border-round p-4">
                        <div class="flex justify-content-between align-items-center mb-5">
                            <span class="text-xl text-900 font-medium">Top Remote Addresses</span>
                        </div>
                        <ul class="list-none p-0 m-0">
                            <div v-for=" host in overview?.top_remote_hosts" :key="host.ip_addr">
                                <li class="flex flex-column md:flex-row md:align-items-center md:justify-content-between mb-4">
                                    <div class="flex">
                                        <div>
                                            <span class="block text-900 font-medium mb-1">{{ host.ip_addr }}</span>
                                            <div class="text-600">{{ host.host_name }}</div>
                                        </div>
                                    </div>
                                    <div class="mt-2 md:mt-0 flex flex-nowrap">
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1">AS{{ host.asn }} {{ host.as_name }}</Button>
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1">{{ host.country_code }}</Button>
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1" icon="pi pi-info-circle"></Button>
                                    </div>
                                </li>
                            </div>
                        </ul>
                    </div>
                </div>
                <div class="col-12 xl:col-6">
                    <div class="surface-card shadow-2 border-round p-4">
                        <div class="text-xl text-900 font-medium mb-4">Top Protocols</div>
                        <ul class="list-none p-0 m-0">
                            <div v-for=" service in overview?.top_app_protocols" :key="service.port">
                                <li class="flex flex-column md:flex-row md:align-items-center md:justify-content-between mb-4">
                                    <div class="flex">
                                        <div>
                                            <span class="block text-900 font-medium mb-1">{{ service.name }}</span>
                                            <div class="text-600">{{ service.port }}/TCP</div>
                                        </div>
                                    </div>
                                    <div class="mt-2 md:mt-0 flex flex-nowrap">
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1">{{ service.traffic.bytes_received }}bytes ↓</Button>
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1">{{ service.traffic.bytes_sent }}bytes ↑</Button>
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1" icon="pi pi-info-circle"></Button>
                                    </div>
                                </li>
                            </div>
                        </ul>
                    </div>
                </div>
                <div class="col-12 xl:col-6">
                    <div class="surface-card shadow-2 border-round p-4">
                        <div class="flex justify-content-between align-items-center mb-5">
                            <span class="text-xl text-900 font-medium">Top Processes</span>
                        </div>
                        <ul class="list-none p-0 m-0">
                            <div v-for=" proc in overview?.top_processes" :key="proc.pid">
                                <li class="flex flex-column md:flex-row md:align-items-center md:justify-content-between mb-4">
                                    <div class="flex">
                                        <div>
                                            <span class="block text-900 font-medium mb-1">{{ proc.name }}</span>
                                            <div class="text-600">{{ proc.pid }}</div>
                                        </div>
                                    </div>
                                    <div class="mt-2 md:mt-0 flex flex-nowrap">
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1">{{ proc.traffic.bytes_received }}bytes ↓</Button>
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1">{{ proc.traffic.bytes_sent }}bytes ↑</Button>
                                        <Button class="p-button-text p-button-plain p-button-rounded mr-1" icon="pi pi-info-circle"></Button>
                                    </div>
                                </li>
                            </div>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>