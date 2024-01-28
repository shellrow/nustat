<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
//import { listen } from '@tauri-apps/api/event';
import { KVItem } from '../types/common';
import { ProcessTrafficInfo } from '../types/np-types';
import { WindowUtil } from '../libnp/window-util';
import { setRoutine } from '../libnp/routine';
import { DataTableRowSelectEvent } from 'primevue/datatable';

const tableData = ref<ProcessTrafficInfo[]>([]);
const selectedHostKv = ref<KVItem[]>([]);
const isLoading = ref(false);
const selectedHost = ref<any>();
const dialogVisible = ref(false);
const windowUtil = new WindowUtil();
const autoUpdate = ref(false);

const routine = setRoutine({
  interval: 5000,
  callback: () => { 
        if (autoUpdate.value) {
            GetProcessInfo(); 
        }
    }
});

const GetProcessInfo = async() => {
    isLoading.value = true;
    const result = await invoke<ProcessTrafficInfo[]>('get_process_info');
    tableData.value = result;
    isLoading.value = false;
}

const onRowSelect = (event: DataTableRowSelectEvent) => {
    const process_info: ProcessTrafficInfo = event.data;
    selectedHostKv.value = [];
    selectedHostKv.value.push({key: 'Process ID', value: process_info.process.pid.toString()});
    selectedHostKv.value.push({key: 'Process Name', value: process_info.process.name});
    selectedHostKv.value.push({key: 'Bytes Sent', value: process_info.traffic.bytes_sent.toString()});
    selectedHostKv.value.push({key: 'Bytes Received', value: process_info.traffic.bytes_received.toString()});
    selectedHostKv.value.push({key: 'Packets Sent', value: process_info.traffic.packet_sent.toString()});
    selectedHostKv.value.push({key: 'Packets Received', value: process_info.traffic.packet_received.toString()});
    selectedHostKv.value.push({key: 'User ID', value: process_info.process.user_info?.user_id.toString() || ''});
    selectedHostKv.value.push({key: 'User Name', value: process_info.process.user_info?.user_name || ''});
    selectedHostKv.value.push({key: 'Group ID', value: process_info.process.user_info?.group_id.toString() || ''});
    selectedHostKv.value.push({key: 'Groups', value: process_info.process.user_info?.groups.join(', ') || ''});
    dialogVisible.value = true;
    console.log(event.data);
};

const onRowUnselect = (event: DataTableRowSelectEvent) => {
    dialogVisible.value = false;
    console.log(event.data);
}

onMounted(() => {
    windowUtil.mount();
    GetProcessInfo();
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
    <Card>
        <template #title> 
            <div class="flex justify-content-between">
                <div class="flex">
                    Network-Active Processes
                </div>
                <div class="flex">
                    <ToggleButton v-model="autoUpdate" onLabel="Auto" offLabel="Manual" onIcon="pi pi-play" offIcon="pi pi-pause" class="mr-2" />
                    <Button type="button" icon="pi pi-refresh" outlined :loading="isLoading" @click="GetProcessInfo" :disabled="autoUpdate" />
                </div>
            </div>
        </template>
        <template #content>
            <DataTable :value="tableData" v-model:selection="selectedHost" :loading="isLoading" :virtualScrollerOptions="{ itemSize: 20 }" selectionMode="single" dataKey="process.pid" @rowSelect="onRowSelect" @rowUnselect="onRowUnselect" size="small" scrollable :scrollHeight="(windowUtil.windowSize.innerHeight-200).toString() + 'px'" tableStyle="min-width: 30rem">
                <Column field="process.pid" header="Process ID" sortable></Column>
                <Column field="process.name" header="Process Name" sortable></Column>
                <Column field="traffic.bytes_sent" header="Bytes Sent" sortable></Column>
                <Column field="traffic.bytes_received" header="Bytes Received" sortable></Column>
                <Column field="traffic.packet_sent" header="Packets Sent" sortable></Column>
                <Column field="traffic.packet_received" header="Packets Received" sortable></Column>
                <Column field="process.user_info.user_name" header="User"></Column>
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
