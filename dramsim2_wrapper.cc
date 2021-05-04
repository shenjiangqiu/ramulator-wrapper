#include "dramsim2_wrapper.h"
#include <iostream>
#include "queue"
#include<functional>
class NCallback : public CallbackBase<void, unsigned, unsigned long long, unsigned long long>
{

public:
    NCallback(std::function<void(unsigned,unsigned long long,unsigned long long)> f) : member(f)
    {
    }

    void operator()(unsigned param1, unsigned long long param2, unsigned long long param3)
    {
        return member(param1, param2, param3);
    }

private:
    const std::function<void(unsigned,unsigned long long,unsigned long long)> member;
};
struct dramsim_wrapper
{
    std::queue<unsigned long long> *return_queue = 0;
    DRAMSim::MultiChannelMemorySystem *m_dramsim = 0;
    NCallback* read_cb=0;
    NCallback* write_cb=0;
};

int dramsim2_get_channel_id(void *dramsim2_wrapper, uint64_t addr)
{
    //auto m_wrapper = (dramsim_wrapper *)dramsim2_wrapper;

    return DRAMSim::MultiChannelMemorySystem::get_channel_id(addr);
}



//this function will be automatically in rust, do not call it mannully
void *get_dramsim2()
{
    dramsim_wrapper *m_wrapper = new dramsim_wrapper;
    m_wrapper->m_dramsim = DRAMSim::getMemorySystemInstance(
        "/Users/jiangqiushen/git/gcn_rust/dramsim2-wrapper/HBM/ini/HBMDevice4GbLegacy.ini",
        "/Users/jiangqiushen/git/gcn_rust/dramsim2-wrapper/HBM/ini/HBMSystemLegacy.ini", ".",
        8192);
    auto read_cb = new NCallback([=](unsigned, unsigned long long addr, unsigned long long) {
        m_wrapper->return_queue->push(addr);
    });
    auto write_cb =new  NCallback([=](unsigned, unsigned long long addr, unsigned long long) {
    });
    m_wrapper->read_cb=read_cb;
    m_wrapper->write_cb=write_cb;
    // m_memory_system->RegisterCallbacks(receive_read, receive_write, NULL);
    m_wrapper->m_dramsim->RegisterCallbacks(read_cb, write_cb, NULL);

    m_wrapper->return_queue = new std::queue<unsigned long long>;
    return m_wrapper;
}
//this function will be automatically called in rust, do not call it mannully
void delete_dramsim2(void *dramsim2_wrapper)
{
    auto m_wrapper = (dramsim_wrapper *)dramsim2_wrapper;

    delete m_wrapper->m_dramsim;
    delete m_wrapper->return_queue;
    delete m_wrapper->write_cb;
    delete m_wrapper->read_cb;
    std::cout << "deleted!!" << std::endl;
    delete m_wrapper;
}

bool dramsim2_send(void *dramsim2_wrapper, unsigned long long addr, bool is_write)
{
    auto m_wrapper = (dramsim_wrapper *)dramsim2_wrapper;
    auto result = m_wrapper->m_dramsim->addTransaction(is_write, addr);
    return result;
}

unsigned long long dramsim2_get(void *dramsim2_wrapper)
{
    auto m_wrapper = (dramsim_wrapper *)dramsim2_wrapper;

    if (m_wrapper->return_queue->size())
    {
        auto result = m_wrapper->return_queue->front();
        m_wrapper->return_queue->pop();
        return result;
    }
    else
    {
        return -1;
    }
}

void dramsim2_tick(void *dramsim2_wrapper)
{
    auto m_wrapper = (dramsim_wrapper *)dramsim2_wrapper;

    m_wrapper->m_dramsim->update();
}

bool dramsim2_ret_available(void *dramsim2_wrapper)
{
    auto m_wrapper = (dramsim_wrapper *)dramsim2_wrapper;
    return m_wrapper->return_queue->size();
}

bool dramsim2_available(void *dramsim2_wrapper, unsigned long long addr, bool is_write)
{
    auto m_wrapper = (dramsim_wrapper *)dramsim2_wrapper;
    return m_wrapper->m_dramsim->willAcceptTransaction(addr);
}