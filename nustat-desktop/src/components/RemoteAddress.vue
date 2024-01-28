<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { RemoteHostInfo } from '../types/np-types';
import { KVItem } from '../types/common';
import { WindowUtil } from '../libnp/window-util';
import { setRoutine } from '../libnp/routine';

const windowUtil = new WindowUtil();
const autoUpdate = ref(false);
const tableData = ref<RemoteHostInfo[]>([]);
const isLoading = ref(false);

const selectedHostKv = ref<KVItem[]>([]);

const selectedHost = ref<RemoteHostInfo>();

const dialogVisible = ref(false);

const onRowSelect = (event: any) => {
    let host: RemoteHostInfo = event.data;
    // set selectedHostKv. order is original order.
    selectedHostKv.value = [];
    selectedHostKv.value.push({key: 'IP Address', value: host.ip_addr});
    selectedHostKv.value.push({key: 'Host Name', value: host.hostname});
    selectedHostKv.value.push({key: 'Packet Sent', value: host.traffic_info.packet_sent.toString()});
    selectedHostKv.value.push({key: 'Packet Received', value: host.traffic_info.packet_received.toString()});
    selectedHostKv.value.push({key: 'Bytes Sent', value: host.traffic_info.bytes_sent.toString()});
    selectedHostKv.value.push({key: 'Bytes Received', value: host.traffic_info.bytes_received.toString()});
    selectedHostKv.value.push({key: 'Country Code', value: host.country_code});
    selectedHostKv.value.push({key: 'Country Name', value: host.country_name});
    selectedHostKv.value.push({key: 'ASN', value: host.asn.toString()});
    selectedHostKv.value.push({key: 'AS Name', value: host.as_name});
    dialogVisible.value = true;
};

const onRowUnselect = (_event: any) => {
    dialogVisible.value = false;
    //console.log(event.data);
}

const GetRemoteHosts = async() => {
    isLoading.value = true;
    const remoteHosts: RemoteHostInfo[] = await invoke('get_remote_hosts');
    tableData.value = remoteHosts;
    isLoading.value = false;
}

const routine = setRoutine({
  interval: 5000,
  callback: () => { 
        if (autoUpdate.value) {
            GetRemoteHosts(); 
        }
    }
});

onMounted(() => {
    windowUtil.mount();
    GetRemoteHosts();
    routine.start();
});

onUnmounted(() => {
    windowUtil.unmount();
    routine.stop();
});

</script>

<style scoped>
.p-card, .p-card-title, .p-card-content {
    background-color: var(--surface-ground);
}
</style>

<template>
    <Card class="flex-auto">
        <template #title> 
            <div class="flex justify-content-between">
                <div class="flex">
                    Detected Remote Address
                </div>
                <div class="flex">
                    <ToggleButton v-model="autoUpdate" onLabel="Auto" offLabel="Manual" onIcon="pi pi-play" offIcon="pi pi-pause" class="mr-2" />
                    <Button type="button" icon="pi pi-refresh" outlined :loading="isLoading" @click="GetRemoteHosts" :disabled="autoUpdate" />
                </div>
            </div>
        </template>
        <template #content>
            <DataTable :value="tableData" v-model:selection="selectedHost" :loading="isLoading" :virtualScrollerOptions="{ itemSize: 20 }" selectionMode="single" dataKey="ip_addr" @rowSelect="onRowSelect" @rowUnselect="onRowUnselect" size="small" scrollable :scrollHeight="(windowUtil.windowSize.innerHeight-200).toString() + 'px'" tableStyle="min-width: 30rem">
                <Column field="ip_addr" header="IP Address" sortable></Column>
                <Column field="hostname" header="Host Name" sortable></Column>
                <Column field="traffic_info.packet_sent" header="Packet Sent" sortable></Column>
                <Column field="traffic_info.packet_received" header="Packet Recv" sortable></Column>
                <Column field="traffic_info.bytes_sent" header="Bytes Sent" sortable></Column>
                <Column field="traffic_info.bytes_received" header="Bytes Recv" sortable></Column>
                <Column field="country_code" header="Country" sortable></Column>
                <Column field="asn" header="ASN" sortable></Column>
            </DataTable>
        </template>
    </Card>
    <Dialog v-model:visible="dialogVisible" :modal="false" :closable="true" header="Detail" :showHeader="true" :breakpoints="{'960px': '75vw', '640px': '100vw'}" :style="{width: '45vw'}">
        <DataTable :value="selectedHostKv"  scrollable scrollHeight="70vh" tableStyle="min-width: 50rem">
                <Column field="key" header="Key" ></Column>
                <Column field="value" header="Value" ></Column>
            </DataTable>
        <template #footer>
            <div class="flex border-top-1 pt-5 surface-border justify-content-end align-items-center">
                <Button @click="dialogVisible = false" icon="pi pi-check" label="OK" class="m-0"></Button>
            </div>
        </template>
    </Dialog>
</template>
