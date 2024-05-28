<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const retrievedData = ref(null)

async function insertData() {
    const data = { id: 1, name: "John Doee", age: 30 }
    const treeName = "users"
    try {
        await invoke('insert_data', { treeName, data })
        alert("Data inserted successfully")
    } catch (error) {
        console.error("Error inserting data:", error)
    }
}

async function getData() {
    const treeName = "users"
    try {
        const response = await invoke('get_data', { treeName, id: 1 })
        retrievedData.value = JSON.parse(response)
        alert(`Data retrieved: ${JSON.stringify(retrievedData.value)}`)
    } catch (error) {
        console.error("Error retrieving data:", error)
    }
}

async function getAllTree() {
    const treeName = "users"
    try {
        const response = await invoke('get_all_data', { treeName })
        retrievedData.value = JSON.parse(response)
        alert(`Data retrieved: ${JSON.stringify(retrievedData.value)}`)
    } catch (error) {
        console.error("Error retrieving data:", error)
    }
}
</script>

<template>
    <div>
        <button @click="insertData">Insert Data</button>
        <button @click="getData">Get Data</button>
        <button @click="getAllTree">Get All Data</button>
        <div v-if="retrievedData">
            <h2>Retrieved Data:</h2>
            <pre>{{ retrievedData }}</pre>
        </div>
    </div>
</template>

<style scoped>
button {
    margin: 10px;
    padding: 10px;
}
</style>